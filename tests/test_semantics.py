"""Reading the definitions of the build-script crate's `shapes` module from Python."""

from pathlib import Path
from types import ModuleType
from typing import Any

from helpers import Loaded, crate, declared, file_id, load, parse


def adt(ra: ModuleType, loaded: Loaded, name: str) -> Any:
    return declared(ra, loaded, "Adt", name)


def names(db: Any, items: list[Any]) -> list[str]:
    return [i.name(db).as_str() for i in items]


def kind(found: Any) -> str:
    """The one variant of a `StructKind`."""
    [only] = [
        name
        for name, read in [("record", found.as_Record()), ("tuple", found.as_Tuple()), ("unit", found.as_Unit())]
        if read
    ]
    return only


def test_an_enum_lists_its_variants_with_their_kind_and_fields(ra: ModuleType, built_loaded: Loaded) -> None:
    db = built_loaded.db
    variants = adt(ra, built_loaded, "Choice").as_enum().variants(db)
    assert names(db, variants) == ["First", "Second", "Third"]
    assert [kind(v.kind(db)) for v in variants] == ["unit", "tuple", "record"]
    assert [names(db, v.fields(db)) for v in variants] == [[], ["0"], ["inner"]]


def test_a_struct_has_its_kind_and_its_fields(ra: ModuleType, built_loaded: Loaded) -> None:
    db = built_loaded.db
    structs = {name: adt(ra, built_loaded, name).as_struct() for name in ("Record", "Pair", "Unit")}
    assert {name: kind(s.kind(db)) for name, s in structs.items()} == {
        "Record": "record",
        "Pair": "tuple",
        "Unit": "unit",
    }
    assert {name: names(db, s.fields(db)) for name, s in structs.items()} == {
        "Record": ["count", "hidden"],
        "Pair": ["0", "1"],
        "Unit": [],
    }


def module_name(db: Any, module: Any) -> str:
    name = module.name(db)
    return "<root>" if name is None else name.as_str()


def modules(ra: ModuleType, loaded: Loaded) -> dict[str, Any]:
    return {module_name(loaded.db, m): m for m in crate(ra, loaded, "src/lib.rs").modules(loaded.db)}


def written(loaded: Loaded, hir_file: Any) -> str:
    """The text of the real file a `HirFileId` stands for."""
    return str(loaded.semantics.deref().parse_guess_edition(hir_file.file_id().file_id(loaded.db)).syntax().text())


def test_a_module_knows_its_name_parent_crate_and_implementations(ra: ModuleType, built_loaded: Loaded) -> None:
    db = built_loaded.db
    found = modules(ra, built_loaded)
    assert set(found) == {"<root>", "shapes", "nested"}  # `pub mod shapes;` in lib.rs, `pub mod nested` in shapes.rs
    parents = {name: (None if (p := m.parent(db)) is None else module_name(db, p)) for name, m in found.items()}
    assert parents == {"<root>": None, "shapes": "<root>", "nested": "shapes"}
    # impl Hook in lib.rs; impl Record, impl Shape for Record, impl Display for Unit in shapes.rs
    assert {name: len(m.impl_defs(db)) for name, m in found.items()} == {"<root>": 1, "shapes": 3, "nested": 0}
    root_file = str(built_loaded.vfs.file_path(crate(ra, built_loaded, "src/lib.rs").root_file(db)).as_path())
    assert all(
        str(built_loaded.vfs.file_path(m.krate(db).root_file(db)).as_path()) == root_file for m in found.values()
    )


def test_a_module_knows_where_it_is_declared_and_defined(ra: ModuleType, built_loaded: Loaded) -> None:
    db = built_loaded.db
    lib, shapes = ((built_loaded.root / "src" / f).read_text() for f in ("lib.rs", "shapes.rs"))
    found = modules(ra, built_loaded)
    assert found["<root>"].declaration_source(db) is None
    declared_shapes = found["shapes"].declaration_source(db)
    assert (str(declared_shapes.value.syntax().text()), written(built_loaded, declared_shapes.file_id)) == (
        "pub mod shapes;",
        lib,
    )
    declared_nested = found["nested"].declaration_source(db)
    assert str(declared_nested.value.syntax().text()).startswith("pub mod nested {")
    assert written(built_loaded, declared_nested.file_id) == shapes
    defined = [
        written(built_loaded, found[name].definition_source(db).file_id) for name in ("<root>", "shapes", "nested")
    ]
    assert defined == [lib, shapes, shapes]  # `nested` is written inline in shapes.rs


def test_a_definition_has_its_module_and_canonical_path(ra: ModuleType, built_loaded: Loaded) -> None:
    db = built_loaded.db
    edition = crate(ra, built_loaded, "src/lib.rs").edition(db)
    found = modules(ra, built_loaded)
    functions = [f.name(db).as_str() for d in found["shapes"].declarations(db) if (f := d.as_Function()) is not None]
    assert functions == ["flows", "waits", "ready"]
    [inside] = found["nested"].declarations(db)
    assert (inside.canonical_path(db, edition), module_name(db, inside.module(db))) == (
        "shapes::nested::inside",
        "nested",
    )
    [flows] = [
        d
        for d in found["shapes"].declarations(db)
        if (f := d.as_Function()) is not None and f.name(db).as_str() == "flows"
    ]
    assert (flows.canonical_path(db, edition), module_name(db, flows.module(db))) == ("shapes::flows", "shapes")


def test_a_path_is_written_in_the_edition_given(ra: ModuleType, built_loaded: Loaded) -> None:
    db = built_loaded.db
    [gen] = [
        d
        for d in modules(ra, built_loaded)["<root>"].declarations(db)
        if (a := d.as_Adt()) is not None and a.name(db).as_str() == "gen"
    ]
    built, core = (
        crate(ra, built_loaded, "src/lib.rs"),
        crate(ra, built_loaded, "src/lib.rs", sysroot="library/core/src/lib.rs"),
    )
    assert gen.canonical_path(db, built.edition(db)) == "gen"  # the crate is edition 2021
    assert gen.canonical_path(db, core.edition(db)) == "r#gen"  # core is edition 2024, where `gen` is a keyword


def test_a_type_or_trait_made_into_a_definition_has_its_path(ra: ModuleType, built_loaded: Loaded) -> None:
    db = built_loaded.db
    edition = crate(ra, built_loaded, "src/lib.rs").edition(db)
    record, shape = adt(ra, built_loaded, "Record"), declared(ra, built_loaded, "Trait", "Shape")
    assert ra.ModuleDef.Adt(record).canonical_path(db, edition) == "shapes::Record"
    assert ra.ModuleDef.Adt(record).as_Adt().name(db).as_str() == "Record"
    assert ra.ModuleDef.Trait(shape).canonical_path(db, edition) == "shapes::Shape"
    assert ra.ModuleDef.Trait(shape).as_Trait().name(db).as_str() == "Shape"


def test_a_crate_has_its_display_name(ra: ModuleType, built_loaded: Loaded) -> None:
    db = built_loaded.db
    built, core = (
        crate(ra, built_loaded, "src/lib.rs"),
        crate(ra, built_loaded, "src/lib.rs", sysroot="library/core/src/lib.rs"),
    )
    assert (str(built.display_name(db)), str(core.display_name(db))) == ("built", "core")


def test_an_expect_lint_attribute_is_read_as_expect(ra: ModuleType, built_loaded: Loaded) -> None:
    built = crate(ra, built_loaded, "src/lib.rs")
    file = file_id(built_loaded, "src/lib.rs")

    def lints(name: str) -> list[tuple[bool, bool, str]]:
        [node] = [
            n
            for n in parse(built_loaded, "src/lib.rs").syntax().descendants()
            if (f := ra.AstFn.cast(n)) is not None and f.name().text() == name
        ]
        return [
            (a.as_Allow(), a.as_Expect(), lint)
            for a, lint in built_loaded.semantics.lint_attrs(file, built, ra.AstAnyHasAttrs.cast(node))
        ]

    assert lints("unused") == [(False, True, "dead_code")]
    assert lints("flagged") == [(False, False, "unused_variables")]  # `deny`; its `cfg_attr` needs the build script


def test_a_node_a_macro_wrote_maps_to_the_name_of_the_macro_at_the_call(ra: ModuleType, built_loaded: Loaded) -> None:
    root = parse(built_loaded, "src/lib.rs").syntax()
    [call] = [m for n in root.descendants() if (m := ra.AstMacroCall.cast(n)) is not None]
    expansion = built_loaded.semantics.deref().expand_macro_call(call)
    [attribute] = [
        a for n in expansion.value.descendants() if (a := ra.AstAttr.cast(n)) is not None
    ]  # `#[allow(unused)]`
    original = ra.InFileSyntaxNode.new(expansion.file_id, attribute.syntax()).original_file_range_rooted(
        built_loaded.db
    )
    text = (built_loaded.root / "src/lib.rs").read_text()
    assert (
        str(
            built_loaded.semantics.deref()
            .parse_guess_edition(original.file_id.file_id(built_loaded.db))
            .syntax()
            .text()
        )
        == text
    )
    start, end = (int(offset) for offset in repr(original.range).split(".."))
    # Its spans lie in the macro's definition, so it falls back to the call: the path and the `!`.
    assert text[start:end] == "make!"


def test_a_place_has_its_line_and_column(ra: ModuleType, built_loaded: Loaded) -> None:
    root = parse(built_loaded, "src/lib.rs").syntax()
    [name] = [n for n in root.descendants() if str(n.text()) == "Always" and repr(n.kind()) == "NAME"]
    place = ra.LineIndex.new((built_loaded.root / "src/lib.rs").read_text()).line_col(name.text_range().start())
    assert (place.line, place.col) == (0, 11)  # `pub struct Always;`


def test_a_node_in_a_real_file_maps_to_its_own_range(ra: ModuleType, built_loaded: Loaded) -> None:
    source = adt(ra, built_loaded, "Always").source(built_loaded.db)
    original = ra.InFileSyntaxNode.new(source.file_id, source.value.syntax()).original_file_range_rooted(
        built_loaded.db
    )
    start, end = (int(offset) for offset in repr(original.range).split(".."))
    assert (built_loaded.root / "src/lib.rs").read_text()[start:end] == "pub struct Always;"


def function_names(db: Any, items: list[Any]) -> list[str]:
    return [f.name(db).as_str() for i in items if (f := i.as_Function()) is not None]


def test_the_implementations_of_a_type_give_their_type_trait_arguments_and_functions(
    ra: ModuleType, built_loaded: Loaded
) -> None:
    db = built_loaded.db
    target = crate(ra, built_loaded, "src/lib.rs").to_display_target(db)

    def implementations(name: str) -> list[tuple[str, str | None, list[str], list[str]]]:
        found: list[tuple[str, str | None, list[str], list[str]]] = []
        for i in ra.Impl.all_for_type(db, adt(ra, built_loaded, name).ty(db)):
            trait, reference = i.trait_(db), i.trait_ref(db)
            # The first argument of a trait reference is the `Self` type.
            arguments = (
                []
                if reference is None
                else [str(a.display(db, target)) for k in range(3) if (a := reference.get_type_argument(k)) is not None]
            )
            found.append(
                (
                    str(i.self_ty(db).display(db, target)),
                    None if trait is None else trait.name(db).as_str(),
                    arguments,
                    function_names(db, i.items(db)),
                )
            )
        return found

    assert implementations("Record") == [("Record", None, [], ["new"]), ("Record", "Shape", ["Record"], ["area"])]
    assert implementations("Unit") == [("Unit", "Display", ["Unit"], ["fmt"])]
    assert implementations("Pair") == []


def test_a_trait_lists_its_functions(ra: ModuleType, built_loaded: Loaded) -> None:
    shape = declared(ra, built_loaded, "Trait", "Shape")
    assert function_names(built_loaded.db, shape.items(built_loaded.db)) == ["area", "label"]


def test_a_type_parameter_lists_its_bounds(ra: ModuleType, built_loaded: Loaded) -> None:
    db = built_loaded.db
    parameters = [
        p.as_type_param(db) for p in ra.GenericDef.from_Adt(adt(ra, built_loaded, "Holder")).type_or_const_params(db)
    ]
    bounds = [
        (p.name(db).as_str(), [t.name(db).as_str() for t in p.trait_bounds(db)]) for p in parameters if p is not None
    ]
    assert bounds == [("T", ["Clone", "Default", "Sized"])]  # `Holder<T: Clone + Default>`; `Sized` is implicit


def test_a_field_knows_its_visibility(ra: ModuleType, built_loaded: Loaded) -> None:
    db = built_loaded.db
    found = modules(ra, built_loaded)
    count, hidden = adt(ra, built_loaded, "Record").as_struct().fields(db)  # `pub count`, `hidden`
    assert (count.visibility(db) == ra.Visibility.Public(), hidden.visibility(db) == ra.Visibility.Public()) == (
        True,
        False,
    )
    assert len({count.visibility(db), hidden.visibility(db), ra.Visibility.Public()}) == 2
    assert [f.is_visible_from(db, found["<root>"]) for f in (count, hidden)] == [True, False]
    assert [f.is_visible_from(db, found["shapes"]) for f in (count, hidden)] == [True, True]


def line(loaded: Loaded, ra: ModuleType, relative: str, node: Any) -> int:
    return ra.LineIndex.new((loaded.root / relative).read_text()).line_col(node.text_range().start()).line + 1


def test_a_file_gives_the_modules_it_defines(ra: ModuleType, built_loaded: Loaded) -> None:
    db = built_loaded.db

    def defined(relative: str) -> list[tuple[str, str]]:
        modules = built_loaded.semantics.file_to_module_defs(file_id(built_loaded, relative))
        return [
            (module_name(db, m), str(built_loaded.vfs.file_path(m.krate(db).root_file(db)).as_path())) for m in modules
        ]

    root = built_loaded.root
    assert defined("src/lib.rs") == [("<root>", str(root / "src/lib.rs"))]
    assert defined("src/shapes.rs") == [("shapes", str(root / "src/lib.rs"))]  # not `nested`, written inside it
    assert defined("build.rs") == [("<root>", str(root / "build.rs"))]  # the build script is a crate of its own
    assert defined("src/loose.rs") == []  # no module declares it


def test_a_method_call_resolves_to_the_function_it_calls(ra: ModuleType, built_loaded: Loaded) -> None:
    db = built_loaded.db
    calls: list[tuple[int, str | None]] = []
    for node in parse(built_loaded, "src/shapes.rs").syntax().descendants():
        if (call := ra.AstMethodCallExpr.cast(node)) is not None:
            function = built_loaded.semantics.deref().resolve_method_call(call)
            calls.append(
                (
                    line(built_loaded, ra, "src/shapes.rs", node),
                    None if function is None else function.name(db).as_str(),
                )
            )
    assert calls == [(62, "first"), (78, "last"), (79, "checked_add"), (81, "area")]


def test_a_called_path_resolves_to_what_it_names(ra: ModuleType, built_loaded: Loaded) -> None:
    db = built_loaded.db

    def called(relative: str) -> list[tuple[int, str]]:
        found: list[tuple[int, str]] = []
        for node in parse(built_loaded, relative).syntax().descendants():
            call = ra.AstCallExpr.cast(node)
            path = None if call is None else call.expr().as_PathExpr()
            if path is None:
                continue
            resolved = built_loaded.semantics.deref().resolve_path(path.path())
            definition = resolved.as_Def()
            if definition is None:
                what = "self type" if resolved.as_SelfType() is not None else "neither"
            elif (function := definition.as_Function()) is not None:
                what = f"function {function.name(db).as_str()}"
            else:
                what = f"variant {definition.as_EnumVariant().name(db).as_str()}"
            found.append((line(built_loaded, ra, relative, node), what))
        return found

    assert called("src/shapes.rs") == [
        (63, "function new"), (64, "function new"), (67, "variant Some"), (81, "function new"), (82, "function new"),
        (87, "variant Some"), (91, "function ready"),
    ]  # fmt: skip
    assert called("src/lib.rs") == [(38, "neither")]  # `run(0)` calls a local binding


def record_patterns(ra: ModuleType, loaded: Loaded, relative: str) -> list[tuple[int, Any]]:
    root = parse(loaded, relative).syntax()
    return [
        (line(loaded, ra, relative, n), p) for n in root.descendants() if (p := ra.AstRecordPat.cast(n)) is not None
    ]


def test_a_record_pattern_resolves_to_the_type_it_takes_apart(ra: ModuleType, built_loaded: Loaded) -> None:
    db = built_loaded.db
    target = crate(ra, built_loaded, "src/lib.rs").to_display_target(db)

    def taken_apart(relative: str) -> list[tuple[int, str]]:
        found: list[tuple[int, str]] = []
        for at, pattern in record_patterns(ra, built_loaded, relative):
            resolved = built_loaded.semantics.deref().resolve_path(pattern.path())
            definition, implementation = resolved.as_Def(), resolved.as_SelfType()
            if definition is not None:
                found.append((at, f"type {definition.as_Adt().name(db).as_str()}"))
            elif implementation is not None:
                found.append((at, f"Self of the impl for {implementation.self_ty(db).display(db, target)}"))
        return found

    assert taken_apart("src/shapes.rs") == [(63, "type Record"), (64, "type Record")]
    assert taken_apart("src/lib.rs") == [(37, "Self of the impl for Hook")]


def test_a_record_pattern_with_rest_or_every_field_leaves_none_out(ra: ModuleType, built_loaded: Loaded) -> None:
    # `Record { count, .. }`, `Record { count: _, hidden }`, `Self { run }`
    patterns = record_patterns(ra, built_loaded, "src/shapes.rs") + record_patterns(ra, built_loaded, "src/lib.rs")
    assert [built_loaded.semantics.deref().record_pattern_missing_fields(p) for _, p in patterns] == [[], [], []]


def test_a_record_pattern_without_rest_lists_the_fields_it_leaves_out(ra: ModuleType, tmp_path: Path) -> None:
    (tmp_path / "Cargo.toml").write_text(
        '[package]\nname = "partial"\nversion = "0.1.0"\nedition = "2021"\n\n[workspace]\n'
    )
    (tmp_path / "src").mkdir()
    (tmp_path / "src" / "lib.rs").write_text(
        "pub struct Point {\n    pub x: u8,\n    pub y: i8,\n}\n\n"
        "// Does not compile: the pattern leaves out `y` and has no `..`.\n"
        "pub fn x(point: Point) -> u8 {\n    let Point { x } = point;\n    x\n}\n"
    )
    loaded = load(ra, tmp_path)
    target = crate(ra, loaded, "src/lib.rs").to_display_target(loaded.db)
    [(_, pattern)] = record_patterns(ra, loaded, "src/lib.rs")
    missing = loaded.semantics.deref().record_pattern_missing_fields(pattern)
    assert [(field.name(loaded.db).as_str(), str(ty.display(loaded.db, target))) for field, ty in missing] == [
        ("y", "i8")
    ]


def test_an_async_function_gives_the_type_it_returns_when_awaited(ra: ModuleType, built_loaded: Loaded) -> None:
    db = built_loaded.db
    target = crate(ra, built_loaded, "src/lib.rs").to_display_target(db)
    found = {
        f.name(db).as_str(): f
        for d in modules(ra, built_loaded)["shapes"].declarations(db)
        if (f := d.as_Function()) is not None
    }
    waits, flows = found["waits"], found["flows"]
    assert (waits.is_async(db), str(waits.async_ret_type(db).display(db, target))) == (True, "u32")
    assert str(waits.ret_type(db).display(db, target)) == "impl Future<Output = u32>"
    assert (flows.is_async(db), flows.async_ret_type(db)) == (False, None)


def marks(ra: ModuleType, loaded: Loaded) -> dict[str, tuple[bool, bool] | None]:
    """For each function of lib.rs, whether it is a test and whether it is ignored; `None` if it is not compiled."""
    found: dict[str, tuple[bool, bool] | None] = {}
    for node in parse(loaded, "src/lib.rs").syntax().descendants():
        if (syntax := ra.AstFn.cast(node)) is not None:
            function = loaded.semantics.deref().to_def(syntax)
            found[syntax.name().text()] = (
                None if function is None else (function.is_test(loaded.db), function.is_ignore(loaded.db))
            )
    return found


def test_a_function_knows_whether_it_is_a_test_and_whether_it_is_ignored(ra: ModuleType, built_tested: Loaded) -> None:
    no = (False, False)
    assert marks(ra, built_tested) == {
        "flagged": no,
        "unused": no,
        "start": no,
        "checks": (True, False),
        "later": (True, True),
    }


def test_tests_are_compiled_only_when_the_crate_is_loaded_for_its_tests(ra: ModuleType, built_loaded: Loaded) -> None:
    found = marks(ra, built_loaded)
    assert (found["checks"], found["later"]) == (None, None)  # `#[cfg(test)]` is off


def test_a_node_maps_to_the_real_file_only_where_it_is_written_there(ra: ModuleType, built_loaded: Loaded) -> None:
    semantics = built_loaded.semantics.deref()
    shapes = parse(built_loaded, "src/shapes.rs").syntax()
    text = (built_loaded.root / "src/shapes.rs").read_text()
    [call] = [m for n in shapes.descendants() if (m := ra.AstMacroCall.cast(n)) is not None]  # `write!(f, "unit")`
    expansion = semantics.expand_macro_call(call)

    def written(node: Any) -> str | None:
        original = semantics.original_range_opt(node)
        if original is None:
            return None
        assert str(semantics.parse_guess_edition(original.file_id.file_id(built_loaded.db)).syntax().text()) == text
        start, end = (int(offset) for offset in repr(original.range).split(".."))
        return text[start:end]

    def first(kind: str) -> Any:
        return next(n for n in expansion.value.descendants() if repr(n.kind()) == kind)

    assert written(first("PATH_EXPR")) == "f"  # from the macro's input
    assert written(first("METHOD_CALL_EXPR")) is None  # `.write_fmt(…)` comes from the macro's definition
    [ready] = [n for n in shapes.descendants() if (f := ra.AstFn.cast(n)) is not None and f.name().text() == "ready"]
    assert written(ready) == "async fn ready() -> u32 {\n    1\n}"


def forms(ra: ModuleType, loaded: Loaded) -> dict[str, Any]:
    """The type of each field of `Forms`, by the field's name."""
    db = loaded.db
    return {f.name(db).as_str(): f.ty(db) for f in adt(ra, loaded, "Forms").as_struct().fields(db)}


def test_a_type_says_which_form_it_has(ra: ModuleType, built_loaded: Loaded) -> None:
    db = built_loaded.db
    target = crate(ra, built_loaded, "src/lib.rs").to_display_target(db)

    def shown(ty: Any) -> str:
        return str(ty.display(db, target))

    found = forms(ra, built_loaded)
    element, length = found["array"].as_array(db)
    assert (shown(element), length) == ("u8", 4)
    to, mutability = found["pointer"].as_raw_ptr()
    assert (shown(to), mutability.is_mut()) == ("u16", True)
    slice, _ = found["slice"].as_reference()
    assert shown(slice.as_slice()) == "u32"
    assert (found["pair"].is_tuple(), [shown(t) for t in found["pair"].tuple_fields(db)]) == (True, ["u8", "i8"])
    assert found["param"].as_type_param(db).name(db).as_str() == "T"
    shape, _ = found["shape"].as_reference()
    assert shape.as_dyn_trait().name(db).as_str() == "Shape"
    plain = found["result"]  # a type of none of these forms
    assert (plain.as_array(db), plain.as_raw_ptr(), plain.as_slice(), plain.is_tuple()) == (None, None, None, False)
    assert (plain.tuple_fields(db), plain.as_type_param(db), plain.as_dyn_trait(), plain.as_impl_traits(db)) == (
        [],
        None,
        None,
        None,
    )


def test_a_type_lists_its_type_arguments(ra: ModuleType, built_loaded: Loaded) -> None:
    db = built_loaded.db
    target = crate(ra, built_loaded, "src/lib.rs").to_display_target(db)
    found = forms(ra, built_loaded)
    assert [str(t.display(db, target)) for t in found["result"].type_arguments()] == ["u8", "String"]
    assert list(found["array"].type_arguments()) == []


def test_a_future_gives_its_output_and_an_iterator_its_item(ra: ModuleType, built_loaded: Loaded) -> None:
    db = built_loaded.db
    target = crate(ra, built_loaded, "src/lib.rs").to_display_target(db)
    [waits] = [
        f
        for d in modules(ra, built_loaded)["shapes"].declarations(db)
        if (f := d.as_Function()) is not None and f.name(db).as_str() == "waits"
    ]
    future = waits.ret_type(db)  # `impl Future<Output = u32>`
    assert [t.name(db).as_str() for t in future.as_impl_traits(db)] == ["Future", "Sized"]  # `Sized` is implicit
    iterator = forms(ra, built_loaded)["range"]  # `std::ops::Range<u32>`
    found = [future.future_output(db), future.iterator_item(db), iterator.iterator_item(db), iterator.future_output(db)]
    # For a type that is no future or no iterator, rust-analyzer gives the projection unresolved, not `None`.
    assert [str(t.display(db, target)) for t in found] == [
        "u32", "<impl Future<Output = u32> as Iterator>::Item", "u32", "<Range<u32> as Future>::Output",
    ]  # fmt: skip


def test_a_variant_pattern_gives_the_enum_it_takes_apart(ra: ModuleType, tmp_path: Path) -> None:
    (tmp_path / "Cargo.toml").write_text(
        '[package]\nname = "mirrored"\nversion = "0.1.0"\nedition = "2021"\n\n[workspace]\n'
    )
    (tmp_path / "src").mkdir()
    (tmp_path / "src" / "lib.rs").write_text(
        "pub enum Model {\n    Bound { details: String },\n}\n\n"
        "pub mod attrs {\n    pub enum ModelAttrs {\n        Bound { details: String },\n    }\n}\n\n"
        "pub fn convert(model: Model) -> attrs::ModelAttrs {\n"
        "    let Model::Bound { details } = model;\n    attrs::ModelAttrs::Bound { details }\n}\n\n"
        "pub fn details(converted: attrs::ModelAttrs) -> String {\n"
        "    let attrs::ModelAttrs::Bound { details } = converted;\n    details\n}\n"
    )
    loaded = load(ra, tmp_path)
    db = loaded.db
    edition = crate(ra, loaded, "src/lib.rs").edition(db)
    found: list[tuple[str, str, str]] = []
    for _, pattern in record_patterns(ra, loaded, "src/lib.rs"):
        variant = loaded.semantics.deref().resolve_path(pattern.path()).as_Def().as_EnumVariant()
        enum = ra.Adt.Enum(variant.parent_enum(db))
        found.append(
            (variant.name(db).as_str(), enum.name(db).as_str(), ra.ModuleDef.Adt(enum).canonical_path(db, edition))
        )
    # The variants alike, their enums told apart by name and path.
    assert found == [("Bound", "Model", "Model"), ("Bound", "ModelAttrs", "attrs::ModelAttrs")]
