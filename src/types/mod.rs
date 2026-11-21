//! One Python class per rust-analyzer type that a bound call takes or returns.
use pyo3::prelude::*;

use crate::database::Database;
use crate::held::Held;

/// `paths::AbsPath`
#[pyclass(unsendable)]
pub struct AbsPath {
    pub inner: Held<ra_ap_paths::AbsPath>,
}

/// `paths::AbsPathBuf`
#[pyclass(unsendable)]
pub struct AbsPathBuf {
    pub inner: ra_ap_paths::AbsPathBuf,
}

/// `hir::Adt`
#[pyclass(unsendable)]
pub struct Adt {
    pub inner: ra_ap_hir::Adt,
}

/// `syntax::ast::Adt`
#[pyclass(unsendable, name = "AstAdt")]
pub struct AstAdt {
    pub inner: Held<ra_ap_syntax::ast::Adt>,
}

/// `hir::Enum`
#[pyclass(unsendable)]
pub struct Enum {
    pub inner: ra_ap_hir::Enum,
}

/// `hir::InFile<ast::Adt>`
#[pyclass(unsendable, name = "InFileAstAdt")]
pub struct InFileAstAdt {
    pub inner: ra_ap_hir::InFile<ra_ap_syntax::ast::Adt>,
}

/// `hir::Module`
#[pyclass(unsendable)]
pub struct Module {
    pub inner: ra_ap_hir::Module,
}

/// `hir::Name`
#[pyclass(unsendable)]
pub struct Name {
    pub inner: ra_ap_hir::Name,
}

/// `hir::Struct`
#[pyclass(unsendable)]
pub struct Struct {
    pub inner: ra_ap_hir::Struct,
}

/// `syntax::SyntaxNode`
#[pyclass(unsendable)]
pub struct SyntaxNode {
    pub inner: Held<ra_ap_syntax::SyntaxNode>,
}

/// `hir::Type<'db>`. It borrows from the database, so it holds the database's Python object,
/// which keeps the database alive as long as the type.
#[pyclass(unsendable)]
pub struct Type {
    pub inner: Held<ra_ap_hir::Type<'static>>,
    pub db: Py<Database>,
}

/// `hir::Crate`
#[pyclass(unsendable)]
pub struct Crate {
    pub inner: ra_ap_hir::Crate,
}

/// `project_model::CargoConfig`
#[pyclass(unsendable)]
pub struct CargoConfig {
    pub inner: ra_ap_project_model::CargoConfig,
}

/// `load_cargo::LoadCargoConfig`
#[pyclass(unsendable)]
pub struct LoadCargoConfig {
    pub inner: ra_ap_load_cargo::LoadCargoConfig,
}

/// `hir::ModuleDef`
#[pyclass(unsendable)]
pub struct ModuleDef {
    pub inner: ra_ap_hir::ModuleDef,
}

/// `proc_macro_api::ProcMacroClient`
#[pyclass(unsendable)]
pub struct ProcMacroClient {
    pub inner: ra_ap_proc_macro_api::ProcMacroClient,
}

/// `load_cargo::ProcMacroServerChoice`
#[pyclass(unsendable)]
pub struct ProcMacroServerChoice {
    pub inner: ra_ap_load_cargo::ProcMacroServerChoice,
}

/// `project_model::ProjectManifest`
#[pyclass(unsendable)]
pub struct ProjectManifest {
    pub inner: ra_ap_project_model::ProjectManifest,
}

/// `project_model::ProjectWorkspace`
#[pyclass(unsendable)]
pub struct ProjectWorkspace {
    pub inner: ra_ap_project_model::ProjectWorkspace,
}

/// `vfs::Vfs`
#[pyclass(unsendable)]
pub struct Vfs {
    pub inner: ra_ap_vfs::Vfs,
}

/// `syntax::ast::AnyHasAttrs`
#[pyclass(unsendable, name = "AstAnyHasAttrs")]
pub struct AnyHasAttrs {
    pub inner: ra_ap_syntax::ast::AnyHasAttrs,
}

/// `syntax::ast::Fn`
#[pyclass(unsendable)]
pub struct AstFn {
    pub inner: Held<ra_ap_syntax::ast::Fn>,
}

/// `project_model::CargoWorkspace`
#[pyclass(unsendable)]
pub struct CargoWorkspace {
    pub inner: Held<ra_ap_project_model::CargoWorkspace>,
}

/// `hir::DisplayTarget`
#[pyclass(unsendable)]
pub struct DisplayTarget {
    pub inner: ra_ap_hir::DisplayTarget,
}

/// `vfs::FileId`
#[pyclass(unsendable)]
pub struct FileId {
    pub inner: ra_ap_vfs::FileId,
}

/// `hir::Function`
#[pyclass(unsendable)]
pub struct Function {
    pub inner: ra_ap_hir::Function,
}

/// `hir::GenericDef`
#[pyclass(unsendable)]
pub struct GenericDef {
    pub inner: ra_ap_hir::GenericDef,
}

/// `syntax::ast::Item`
#[pyclass(unsendable, name = "AstItem")]
pub struct Item {
    pub inner: ra_ap_syntax::ast::Item,
}

/// `line_index::LineCol`
#[pyclass(unsendable)]
pub struct LineCol {
    pub inner: ra_ap_ide_db::line_index::LineCol,
}

/// `line_index::LineIndex`
#[pyclass(unsendable)]
pub struct LineIndex {
    pub inner: ra_ap_ide_db::line_index::LineIndex,
}

/// `hir::LintAttr`
#[pyclass(unsendable)]
pub struct LintAttr {
    pub inner: ra_ap_hir::LintAttr,
}

/// `hir::Mutability`
#[pyclass(unsendable)]
pub struct Mutability {
    pub inner: ra_ap_hir::Mutability,
}

/// `syntax::SyntaxElement`, which is `NodeOrToken<SyntaxNode, SyntaxToken>`
#[pyclass(unsendable)]
pub struct SyntaxElement {
    pub inner: ra_ap_syntax::SyntaxElement,
}

/// `project_model::Package`
#[pyclass(unsendable)]
pub struct Package {
    pub inner: ra_ap_project_model::Package,
}

/// `project_model::PackageData`
#[pyclass(unsendable)]
pub struct PackageData {
    pub inner: Held<ra_ap_project_model::PackageData>,
}

/// `project_model::ProjectWorkspaceKind`
#[pyclass(unsendable)]
pub struct ProjectWorkspaceKind {
    pub inner: ra_ap_project_model::ProjectWorkspaceKind,
}

/// `syntax::ast::SourceFile`
#[pyclass(unsendable, name = "AstSourceFile")]
pub struct SourceFile {
    pub inner: ra_ap_syntax::ast::SourceFile,
}

/// `parser::SyntaxKind`
#[pyclass(unsendable)]
pub struct SyntaxKind {
    pub inner: ra_ap_syntax::SyntaxKind,
}

/// `rowan::SyntaxText`
#[pyclass(unsendable)]
pub struct SyntaxText {
    pub inner: ra_ap_syntax::SyntaxText,
}

/// `rowan::SyntaxToken`
#[pyclass(unsendable)]
pub struct SyntaxToken {
    pub inner: ra_ap_syntax::SyntaxToken,
}

/// `text_size::TextRange`
#[pyclass(unsendable)]
pub struct TextRange {
    pub inner: ra_ap_syntax::TextRange,
}

/// `text_size::TextSize`
#[pyclass(unsendable)]
pub struct TextSize {
    pub inner: ra_ap_syntax::TextSize,
}

/// `hir::Trait`
#[pyclass(unsendable)]
pub struct Trait {
    pub inner: ra_ap_hir::Trait,
}

/// `hir::TypeOrConstParam`
#[pyclass(unsendable)]
pub struct TypeOrConstParam {
    pub inner: ra_ap_hir::TypeOrConstParam,
}

/// `hir::TypeParam`
#[pyclass(unsendable)]
pub struct TypeParam {
    pub inner: ra_ap_hir::TypeParam,
}

/// `vfs::VfsPath`
#[pyclass(unsendable)]
pub struct VfsPath {
    pub inner: Held<ra_ap_vfs::VfsPath>,
}

/// `project_model::WorkspaceBuildScripts`
#[pyclass(unsendable)]
pub struct WorkspaceBuildScripts {
    pub inner: ra_ap_project_model::WorkspaceBuildScripts,
}

/// `project_model::RustLibSource`
#[pyclass(unsendable)]
pub struct RustLibSource {
    pub inner: ra_ap_project_model::RustLibSource,
}

/// `hir::InFile<ast::Fn>`
#[pyclass(unsendable, name = "InFileAstFn")]
pub struct InFileFn {
    pub inner: ra_ap_hir::InFile<ra_ap_syntax::ast::Fn>,
}

/// `hir::InFile<TextRange>`
#[pyclass(unsendable, name = "InFileTextRange")]
pub struct InFileTextRange {
    pub inner: ra_ap_hir::InFile<ra_ap_syntax::TextRange>,
}

/// `hir::HirFileId`
#[pyclass(unsendable)]
pub struct HirFileId {
    pub inner: ra_ap_hir::HirFileId,
}

/// `hir::EditionedFileId`
#[pyclass(unsendable)]
pub struct EditionedFileId {
    pub inner: ra_ap_hir::EditionedFileId,
}

/// `hir::FileRange`, which is `FileRangeWrapper<EditionedFileId>`
#[pyclass(unsendable)]
pub struct FileRange {
    pub inner: ra_ap_hir::FileRange,
}

/// `project_model::PackageDependency`
#[pyclass(unsendable)]
pub struct PackageDependency {
    pub inner: Held<ra_ap_project_model::PackageDependency>,
}

/// `project_model::DepKind`. rust-analyzer does not export the type, so it cannot be named outside
/// rust-analyzer; the class holds the `PackageDependency` whose `kind` it is.
#[pyclass(unsendable)]
pub struct DepKind {
    pub dependency: Held<ra_ap_project_model::PackageDependency>,
}

/// `hir::Field`
#[pyclass(unsendable)]
pub struct Field {
    pub inner: ra_ap_hir::Field,
}

/// `hir::Callable<'db>`. It borrows the database, so it holds the database's Python object.
#[pyclass(unsendable)]
pub struct Callable {
    pub inner: ra_ap_hir::Callable<'static>,
    pub db: Py<Database>,
}

/// `hir::Param<'db>`. It borrows the database, so it holds the database's Python object.
#[pyclass(unsendable)]
pub struct Param {
    pub inner: ra_ap_hir::Param<'static>,
    pub db: Py<Database>,
}

/// `syntax::ast::MacroCall`
#[pyclass(unsendable)]
pub struct AstMacroCall {
    pub inner: ra_ap_syntax::ast::MacroCall,
}

/// `hir::InFile<SyntaxNode>`
#[pyclass(unsendable)]
pub struct InFileSyntaxNode {
    pub inner: ra_ap_hir::InFile<ra_ap_syntax::SyntaxNode>,
}

/// `hir::MacroCallId`
#[pyclass(unsendable)]
pub struct MacroCallId {
    pub inner: ra_ap_hir::MacroCallId,
}

/// `hir_expand::InMacroFile<SyntaxToken>`
#[pyclass(unsendable)]
pub struct InMacroFileSyntaxToken {
    pub inner: ra_ap_hir_expand::InMacroFile<ra_ap_syntax::SyntaxToken>,
}

/// `hir::InFile<SmallVec<[TextRange; 1]>>`
#[pyclass(unsendable)]
pub struct InFileSmallVecTextRange {
    pub inner: ra_ap_hir::InFile<smallvec::SmallVec<[ra_ap_syntax::TextRange; 1]>>,
}

/// The variant `ProjectWorkspaceKind::Cargo`, lent by the `ProjectWorkspaceKind` that is that variant.
#[pyclass(unsendable)]
pub struct ProjectWorkspaceKindCargo {
    pub kind: &'static ra_ap_project_model::ProjectWorkspaceKind,
    pub _owner: Py<PyAny>,
}

/// `hir::Semantics<'db, RootDatabase>`. It borrows the database, so it holds the database's Python object.
#[pyclass(unsendable, name = "SemanticsRootDatabase")]
pub struct Semantics {
    pub inner: ra_ap_hir::Semantics<'static, ra_ap_ide_db::RootDatabase>,
    pub db: Py<Database>,
}

/// `hir::SemanticsImpl<'db>`, lent by a `Semantics`.
#[pyclass(unsendable)]
pub struct SemanticsImpl {
    pub inner: &'static ra_ap_hir::SemanticsImpl<'static>,
    pub db: Py<Database>,
    pub _owner: Py<PyAny>,
}

/// `hir_ty::display::HirDisplayWrapper<'a, 'db, Type<'db>>`. It borrows the type it displays and the
/// database, so it holds the Python objects of both.
#[pyclass(unsendable, name = "HirDisplayWrapperType")]
pub struct HirDisplayWrapper {
    pub inner: ra_ap_hir_ty::display::HirDisplayWrapper<'static, 'static, ra_ap_hir::Type<'static>>,
    pub db: Py<Database>,
    pub _owner: Py<PyAny>,
}

pub fn register(module: &Bound<'_, PyModule>) -> PyResult<()> {
    module.add_class::<AbsPath>()?;
    module.add_class::<AbsPathBuf>()?;
    module.add_class::<Adt>()?;
    module.add_class::<AstAdt>()?;
    module.add_class::<Enum>()?;
    module.add_class::<InFileAstAdt>()?;
    module.add_class::<Module>()?;
    module.add_class::<Name>()?;
    module.add_class::<Struct>()?;
    module.add_class::<SyntaxNode>()?;
    module.add_class::<Type>()?;
    module.add_class::<Crate>()?;
    module.add_class::<CargoConfig>()?;
    module.add_class::<LoadCargoConfig>()?;
    module.add_class::<ModuleDef>()?;
    module.add_class::<ProcMacroClient>()?;
    module.add_class::<ProcMacroServerChoice>()?;
    module.add_class::<ProjectManifest>()?;
    module.add_class::<ProjectWorkspace>()?;
    module.add_class::<Vfs>()?;
    module.add_class::<AnyHasAttrs>()?;
    module.add_class::<AstFn>()?;
    module.add_class::<CargoWorkspace>()?;
    module.add_class::<DisplayTarget>()?;
    module.add_class::<FileId>()?;
    module.add_class::<Function>()?;
    module.add_class::<GenericDef>()?;
    module.add_class::<Item>()?;
    module.add_class::<LineCol>()?;
    module.add_class::<LineIndex>()?;
    module.add_class::<LintAttr>()?;
    module.add_class::<Mutability>()?;
    module.add_class::<SyntaxElement>()?;
    module.add_class::<Package>()?;
    module.add_class::<PackageData>()?;
    module.add_class::<ProjectWorkspaceKind>()?;
    module.add_class::<SourceFile>()?;
    module.add_class::<SyntaxKind>()?;
    module.add_class::<SyntaxText>()?;
    module.add_class::<SyntaxToken>()?;
    module.add_class::<TextRange>()?;
    module.add_class::<TextSize>()?;
    module.add_class::<Trait>()?;
    module.add_class::<TypeOrConstParam>()?;
    module.add_class::<TypeParam>()?;
    module.add_class::<VfsPath>()?;
    module.add_class::<WorkspaceBuildScripts>()?;
    module.add_class::<ProjectWorkspaceKindCargo>()?;
    module.add_class::<Semantics>()?;
    module.add_class::<SemanticsImpl>()?;
    module.add_class::<HirDisplayWrapper>()?;
    module.add_class::<RustLibSource>()?;
    module.add_class::<InFileFn>()?;
    module.add_class::<InFileTextRange>()?;
    module.add_class::<HirFileId>()?;
    module.add_class::<EditionedFileId>()?;
    module.add_class::<FileRange>()?;
    module.add_class::<PackageDependency>()?;
    module.add_class::<DepKind>()?;
    module.add_class::<Field>()?;
    module.add_class::<Callable>()?;
    module.add_class::<Param>()?;
    module.add_class::<AstMacroCall>()?;
    module.add_class::<InFileSyntaxNode>()?;
    module.add_class::<MacroCallId>()?;
    module.add_class::<InMacroFileSyntaxToken>()?;
    module.add_class::<InFileSmallVecTextRange>()?;
    module.add_class::<crate::iter::Iter>()?;
    Ok(())
}
