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
    pub inner: Held<ra_ap_syntax::SyntaxToken>,
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

/// `syntax::ast::ArgList`
#[pyclass(unsendable)]
pub struct AstArgList {
    pub inner: ra_ap_syntax::ast::ArgList,
}

/// `syntax::ast::AwaitExpr`
#[pyclass(unsendable)]
pub struct AstAwaitExpr {
    pub inner: Held<ra_ap_syntax::ast::AwaitExpr>,
}

/// `syntax::ast::BlockExpr`
#[pyclass(unsendable)]
pub struct AstBlockExpr {
    pub inner: Held<ra_ap_syntax::ast::BlockExpr>,
}

/// `syntax::ast::ClosureExpr`
#[pyclass(unsendable)]
pub struct AstClosureExpr {
    pub inner: Held<ra_ap_syntax::ast::ClosureExpr>,
}

/// `syntax::ast::ExprStmt`
#[pyclass(unsendable)]
pub struct AstExprStmt {
    pub inner: ra_ap_syntax::ast::ExprStmt,
}

/// `syntax::ast::ExternCrate`
#[pyclass(unsendable)]
pub struct AstExternCrate {
    pub inner: ra_ap_syntax::ast::ExternCrate,
}

/// `syntax::ast::ItemList`
#[pyclass(unsendable)]
pub struct AstItemList {
    pub inner: ra_ap_syntax::ast::ItemList,
}

/// `syntax::ast::LetExpr`
#[pyclass(unsendable)]
pub struct AstLetExpr {
    pub inner: ra_ap_syntax::ast::LetExpr,
}

/// `syntax::ast::MatchArmList`
#[pyclass(unsendable)]
pub struct AstMatchArmList {
    pub inner: ra_ap_syntax::ast::MatchArmList,
}

/// `syntax::ast::MatchExpr`
#[pyclass(unsendable)]
pub struct AstMatchExpr {
    pub inner: Held<ra_ap_syntax::ast::MatchExpr>,
}

/// `syntax::ast::MethodCallExpr`
#[pyclass(unsendable)]
pub struct AstMethodCallExpr {
    pub inner: ra_ap_syntax::ast::MethodCallExpr,
}

/// `syntax::ast::ParenExpr`
#[pyclass(unsendable)]
pub struct AstParenExpr {
    pub inner: Held<ra_ap_syntax::ast::ParenExpr>,
}

/// `syntax::ast::MacroRules`
#[pyclass(unsendable)]
pub struct AstMacroRules {
    pub inner: Held<ra_ap_syntax::ast::MacroRules>,
}

/// `syntax::ast::RefExpr`
#[pyclass(unsendable)]
pub struct AstRefExpr {
    pub inner: ra_ap_syntax::ast::RefExpr,
}

/// `syntax::ast::ReturnExpr`
#[pyclass(unsendable)]
pub struct AstReturnExpr {
    pub inner: ra_ap_syntax::ast::ReturnExpr,
}

/// `syntax::ast::TryExpr`
#[pyclass(unsendable)]
pub struct AstTryExpr {
    pub inner: Held<ra_ap_syntax::ast::TryExpr>,
}

/// `syntax::ast::Use`
#[pyclass(unsendable)]
pub struct AstUse {
    pub inner: ra_ap_syntax::ast::Use,
}

/// `syntax::ast::AssocItemList`
#[pyclass(unsendable)]
pub struct AstAssocItemList {
    pub inner: ra_ap_syntax::ast::AssocItemList,
}

/// `syntax::ast::Attr`
#[pyclass(unsendable)]
pub struct AstAttr {
    pub inner: ra_ap_syntax::ast::Attr,
}

/// `syntax::ast::CallExpr`
#[pyclass(unsendable)]
pub struct AstCallExpr {
    pub inner: ra_ap_syntax::ast::CallExpr,
}

/// `syntax::ast::Expr`
#[pyclass(unsendable)]
pub struct AstExpr {
    pub inner: ra_ap_syntax::ast::Expr,
}

/// `syntax::ast::FieldExpr`
#[pyclass(unsendable)]
pub struct AstFieldExpr {
    pub inner: ra_ap_syntax::ast::FieldExpr,
}

/// `syntax::ast::LetStmt`
#[pyclass(unsendable)]
pub struct AstLetStmt {
    pub inner: ra_ap_syntax::ast::LetStmt,
}

/// `syntax::ast::Module`
#[pyclass(unsendable)]
pub struct AstModule {
    pub inner: Held<ra_ap_syntax::ast::Module>,
}

/// `syntax::ast::RecordFieldList`
#[pyclass(unsendable)]
pub struct AstRecordFieldList {
    pub inner: ra_ap_syntax::ast::RecordFieldList,
}

/// `syntax::ast::RecordPat`
#[pyclass(unsendable)]
pub struct AstRecordPat {
    pub inner: ra_ap_syntax::ast::RecordPat,
}

/// `syntax::ast::StmtList`
#[pyclass(unsendable)]
pub struct AstStmtList {
    pub inner: ra_ap_syntax::ast::StmtList,
}

/// `syntax::ast::VariantList`
#[pyclass(unsendable)]
pub struct AstVariantList {
    pub inner: ra_ap_syntax::ast::VariantList,
}

/// `syntax::ast::ForExpr`
#[pyclass(unsendable)]
pub struct AstForExpr {
    pub inner: Held<ra_ap_syntax::ast::ForExpr>,
}

/// `syntax::ast::IfExpr`
#[pyclass(unsendable)]
pub struct AstIfExpr {
    pub inner: Held<ra_ap_syntax::ast::IfExpr>,
}

/// `syntax::ast::LoopExpr`
#[pyclass(unsendable)]
pub struct AstLoopExpr {
    pub inner: Held<ra_ap_syntax::ast::LoopExpr>,
}

/// `syntax::ast::PathExpr`
#[pyclass(unsendable)]
pub struct AstPathExpr {
    pub inner: Held<ra_ap_syntax::ast::PathExpr>,
}

/// `syntax::ast::WhileExpr`
#[pyclass(unsendable)]
pub struct AstWhileExpr {
    pub inner: Held<ra_ap_syntax::ast::WhileExpr>,
}

/// `syntax::ast::IdentPat`
#[pyclass(unsendable)]
pub struct AstIdentPat {
    pub inner: Held<ra_ap_syntax::ast::IdentPat>,
}

/// `syntax::ast::WildcardPat`
#[pyclass(unsendable)]
pub struct AstWildcardPat {
    pub inner: Held<ra_ap_syntax::ast::WildcardPat>,
}

/// `syntax::ast::Stmt`
#[pyclass(unsendable)]
pub struct AstStmt {
    pub inner: ra_ap_syntax::ast::Stmt,
}

/// `syntax::ast::Pat`
#[pyclass(unsendable)]
pub struct AstPat {
    pub inner: ra_ap_syntax::ast::Pat,
}

/// `syntax::ast::Path`
#[pyclass(unsendable)]
pub struct AstPath {
    pub inner: ra_ap_syntax::ast::Path,
}

/// `syntax::ast::CfgAttrMeta`
#[pyclass(unsendable)]
pub struct AstCfgAttrMeta {
    pub inner: Held<ra_ap_syntax::ast::CfgAttrMeta>,
}

/// `syntax::ast::KeyValueMeta`
#[pyclass(unsendable)]
pub struct AstKeyValueMeta {
    pub inner: Held<ra_ap_syntax::ast::KeyValueMeta>,
}

/// `syntax::ast::PathMeta`
#[pyclass(unsendable)]
pub struct AstPathMeta {
    pub inner: Held<ra_ap_syntax::ast::PathMeta>,
}

/// `syntax::ast::TokenTreeMeta`
#[pyclass(unsendable)]
pub struct AstTokenTreeMeta {
    pub inner: Held<ra_ap_syntax::ast::TokenTreeMeta>,
}

/// `syntax::ast::Meta`
#[pyclass(unsendable)]
pub struct AstMeta {
    pub inner: ra_ap_syntax::ast::Meta,
}

/// `syntax::ast::TokenTree`
#[pyclass(unsendable)]
pub struct AstTokenTree {
    pub inner: ra_ap_syntax::ast::TokenTree,
}

/// `syntax::ast::CfgPredicate`
#[pyclass(unsendable)]
pub struct AstCfgPredicate {
    pub inner: ra_ap_syntax::ast::CfgPredicate,
}

/// `syntax::ast::Name`
#[pyclass(unsendable)]
pub struct AstName {
    pub inner: ra_ap_syntax::ast::Name,
}

/// `syntax::ast::Comment`
#[pyclass(unsendable)]
pub struct AstComment {
    pub inner: ra_ap_syntax::ast::Comment,
}

/// `syntax::ast::AssocItem`
#[pyclass(unsendable)]
pub struct AstAssocItem {
    pub inner: ra_ap_syntax::ast::AssocItem,
}

/// `syntax::ast::RecordField`
#[pyclass(unsendable)]
pub struct AstRecordField {
    pub inner: ra_ap_syntax::ast::RecordField,
}

/// `syntax::ast::Variant`
#[pyclass(unsendable)]
pub struct AstVariant {
    pub inner: ra_ap_syntax::ast::Variant,
}

/// `syntax::ast::DocComment`
#[pyclass(unsendable)]
pub struct AstDocComment {
    pub inner: ra_ap_syntax::ast::DocComment,
}

/// `syntax::ast::RecordPatFieldList`
#[pyclass(unsendable)]
pub struct AstRecordPatFieldList {
    pub inner: ra_ap_syntax::ast::RecordPatFieldList,
}

/// `syntax::ast::RecordPatField`
#[pyclass(unsendable)]
pub struct AstRecordPatField {
    pub inner: ra_ap_syntax::ast::RecordPatField,
}

/// `syntax::ast::RestPat`
#[pyclass(unsendable)]
pub struct AstRestPat {
    pub inner: ra_ap_syntax::ast::RestPat,
}

/// `hir::EnumVariant`
#[pyclass(unsendable)]
pub struct EnumVariant {
    pub inner: ra_ap_hir::EnumVariant,
}

/// `hir::StructKind`
#[pyclass(unsendable)]
pub struct StructKind {
    pub inner: ra_ap_hir::StructKind,
}

/// `syntax::Edition`
#[pyclass(unsendable)]
pub struct Edition {
    pub inner: ra_ap_syntax::Edition,
}

/// `base_db::CrateDisplayName`
#[pyclass(unsendable)]
pub struct CrateDisplayName {
    pub inner: ra_ap_ide_db::base_db::CrateDisplayName,
}

/// `hir::Impl`
#[pyclass(unsendable)]
pub struct Impl {
    pub inner: ra_ap_hir::Impl,
}

/// `hir::InFile<ast::Module>`
#[pyclass(unsendable)]
pub struct InFileAstModule {
    pub inner: ra_ap_hir::InFile<ra_ap_syntax::ast::Module>,
}

/// `hir::InFile<ModuleSource>`
#[pyclass(unsendable)]
pub struct InFileModuleSource {
    pub inner: ra_ap_hir::InFile<ra_ap_hir::ModuleSource>,
}

/// `hir::AssocItem`
#[pyclass(unsendable)]
pub struct AssocItem {
    pub inner: ra_ap_hir::AssocItem,
}

/// `hir::TraitRef<'db>`. It borrows the database, so it holds the database's Python object.
#[pyclass(unsendable)]
pub struct TraitRef {
    pub inner: ra_ap_hir::TraitRef<'static>,
    pub db: Py<Database>,
}

/// `hir::Visibility`
#[pyclass(unsendable)]
pub struct Visibility {
    pub inner: ra_ap_hir::Visibility,
}

/// `project_model::Target`, which is `Idx<TargetData>`
#[pyclass(unsendable)]
pub struct Target {
    pub inner: ra_ap_project_model::Target,
}

/// `project_model::TargetData`
#[pyclass(unsendable)]
pub struct TargetData {
    pub inner: Held<ra_ap_project_model::TargetData>,
}

/// `hir::BuiltinType`
#[pyclass(unsendable)]
pub struct BuiltinType {
    pub inner: ra_ap_hir::BuiltinType,
}

/// `project_model::TargetKind`
#[pyclass(unsendable)]
pub struct TargetKind {
    pub inner: ra_ap_project_model::TargetKind,
}

/// `project_model::ManifestPath`
#[pyclass(unsendable)]
pub struct ManifestPath {
    pub inner: Held<ra_ap_project_model::ManifestPath>,
}

/// `hir::PathResolution<'db>`. It borrows the database, so it holds the database's Python object.
#[pyclass(unsendable)]
pub struct PathResolution {
    pub inner: ra_ap_hir::PathResolution<'static>,
    pub _db: Py<Database>,
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

/// `hir_ty::display::HirDisplayWrapper<'a, 'db, TraitRef<'db>>`. It borrows the trait reference it displays
/// and the database, so it holds the Python objects of both.
#[pyclass(unsendable)]
pub struct HirDisplayWrapperTraitRef {
    pub inner: ra_ap_hir_ty::display::HirDisplayWrapper<'static, 'static, ra_ap_hir::TraitRef<'static>>,
    pub db: Py<Database>,
    pub _owner: Py<PyAny>,
}

/// `project_model::CargoFeatures`
#[pyclass(unsendable)]
pub struct CargoFeatures {
    pub inner: ra_ap_project_model::CargoFeatures,
}

/// `project_model::CfgOverrides`
#[pyclass(unsendable)]
pub struct CfgOverrides {
    pub inner: ra_ap_project_model::CfgOverrides,
}

/// `cfg::CfgDiff`
#[pyclass(unsendable)]
pub struct CfgDiff {
    pub inner: ra_ap_cfg::CfgDiff,
}

/// `cfg::CfgAtom`
#[pyclass(unsendable)]
pub struct CfgAtom {
    pub inner: ra_ap_hir::CfgAtom,
}

/// `cfg::CfgExpr`
#[pyclass(unsendable)]
pub struct CfgExpr {
    pub inner: ra_ap_hir::CfgExpr,
}

/// `cfg::CfgOptions`
#[pyclass(unsendable)]
pub struct CfgOptions {
    pub inner: ra_ap_hir::CfgOptions,
}

/// `intern::Symbol`
#[pyclass(unsendable)]
pub struct Symbol {
    pub inner: ra_ap_hir::Symbol,
}

/// `project_model::TargetDirectoryConfig`
#[pyclass(unsendable)]
pub struct TargetDirectoryConfig {
    pub inner: ra_ap_project_model::TargetDirectoryConfig,
}

/// `project_model::Sysroot`
#[pyclass(unsendable)]
pub struct Sysroot {
    pub inner: ra_ap_project_model::Sysroot,
}

/// `hir::Union`
#[pyclass(unsendable)]
pub struct Union {
    pub inner: ra_ap_hir::Union,
}

/// `syntax::ast::Impl`
#[pyclass(unsendable)]
pub struct AstImpl {
    pub inner: Held<ra_ap_syntax::ast::Impl>,
}

/// `syntax::ast::Trait`
#[pyclass(unsendable)]
pub struct AstTrait {
    pub inner: Held<ra_ap_syntax::ast::Trait>,
}

/// `syntax::ast::TupleField`
#[pyclass(unsendable)]
pub struct AstTupleField {
    pub inner: ra_ap_syntax::ast::TupleField,
}

/// `syntax::ast::NameRef`
#[pyclass(unsendable)]
pub struct AstNameRef {
    pub inner: ra_ap_syntax::ast::NameRef,
}

/// `syntax::ast::CfgMeta`
#[pyclass(unsendable)]
pub struct AstCfgMeta {
    pub inner: Held<ra_ap_syntax::ast::CfgMeta>,
}

/// `hir::InFile<ast::Impl>`
#[pyclass(unsendable)]
pub struct InFileAstImpl {
    pub inner: ra_ap_hir::InFile<ra_ap_syntax::ast::Impl>,
}

/// `hir::InFile<ast::Trait>`
#[pyclass(unsendable)]
pub struct InFileAstTrait {
    pub inner: ra_ap_hir::InFile<ra_ap_syntax::ast::Trait>,
}

/// `hir::InFile<ast::Variant>`
#[pyclass(unsendable)]
pub struct InFileAstVariant {
    pub inner: ra_ap_hir::InFile<ra_ap_syntax::ast::Variant>,
}

/// `hir::InFile<FieldSource>`
#[pyclass(unsendable)]
pub struct InFileFieldSource {
    pub inner: ra_ap_hir::InFile<ra_ap_hir::FieldSource>,
}

/// `hir::FieldSource`, lent by the `InFile` that holds it: the type cannot be copied.
#[pyclass(unsendable)]
pub struct FieldSource {
    pub inner: &'static ra_ap_hir::FieldSource,
    pub _owner: Py<PyAny>,
}

/// `hir::TypeInfo<'db>`. It borrows the database, so it holds the database's Python object.
#[pyclass(unsendable)]
pub struct TypeInfo {
    pub inner: ra_ap_hir::TypeInfo<'static>,
    pub db: Py<Database>,
}

/// `hir::Docs`
#[pyclass(unsendable)]
pub struct Docs {
    pub inner: ra_ap_hir::Docs,
}

/// `line_index::WideEncoding`
#[pyclass(unsendable)]
pub struct WideEncoding {
    pub inner: ra_ap_ide_db::line_index::WideEncoding,
}

/// `line_index::WideLineCol`
#[pyclass(unsendable)]
pub struct WideLineCol {
    pub inner: ra_ap_ide_db::line_index::WideLineCol,
}

/// `hir::GenericParam`
#[pyclass(unsendable)]
pub struct GenericParam {
    pub inner: ra_ap_hir::GenericParam,
}

/// `syntax::SyntaxError`
#[pyclass(unsendable)]
pub struct SyntaxError {
    pub inner: ra_ap_syntax::SyntaxError,
}

/// `hir::Variant`
#[pyclass(unsendable)]
pub struct Variant {
    pub inner: ra_ap_hir::Variant,
}

/// `hir::TypeAlias`
#[pyclass(unsendable)]
pub struct TypeAlias {
    pub inner: ra_ap_hir::TypeAlias,
}

/// `Either<hir::Field, hir::TupleField<'db>>`. It borrows from the database, so it holds the database's Python
/// object.
#[pyclass(unsendable)]
pub struct EitherFieldTupleField {
    pub inner: either::Either<ra_ap_hir::Field, ra_ap_hir::TupleField<'static>>,
    pub _db: Py<Database>,
}

/// `syntax::ast::ParamList`
#[pyclass(unsendable)]
pub struct AstParamList {
    pub inner: ra_ap_syntax::ast::ParamList,
}

/// `syntax::ast::Param`
#[pyclass(unsendable)]
pub struct AstParam {
    pub inner: ra_ap_syntax::ast::Param,
}

/// `syntax::ast::MatchArm`
#[pyclass(unsendable)]
pub struct AstMatchArm {
    pub inner: ra_ap_syntax::ast::MatchArm,
}

/// `syntax::ast::MatchGuard`
#[pyclass(unsendable)]
pub struct AstMatchGuard {
    pub inner: ra_ap_syntax::ast::MatchGuard,
}

/// `syntax::ast::LetElse`
#[pyclass(unsendable)]
pub struct AstLetElse {
    pub inner: ra_ap_syntax::ast::LetElse,
}

/// `syntax::ast::BinExpr`
#[pyclass(unsendable)]
pub struct AstBinExpr {
    pub inner: ra_ap_syntax::ast::BinExpr,
}

/// `syntax::ast::ArrayExpr`
#[pyclass(unsendable)]
pub struct AstArrayExpr {
    pub inner: ra_ap_syntax::ast::ArrayExpr,
}

/// `syntax::ast::TupleStructPat`
#[pyclass(unsendable)]
pub struct AstTupleStructPat {
    pub inner: ra_ap_syntax::ast::TupleStructPat,
}

/// `syntax::ast::TuplePat`
#[pyclass(unsendable)]
pub struct AstTuplePat {
    pub inner: ra_ap_syntax::ast::TuplePat,
}

/// `syntax::ast::SlicePat`
#[pyclass(unsendable)]
pub struct AstSlicePat {
    pub inner: ra_ap_syntax::ast::SlicePat,
}

/// `syntax::ast::OrPat`
#[pyclass(unsendable)]
pub struct AstOrPat {
    pub inner: ra_ap_syntax::ast::OrPat,
}

/// `syntax::ast::RefPat`
#[pyclass(unsendable)]
pub struct AstRefPat {
    pub inner: ra_ap_syntax::ast::RefPat,
}

/// `syntax::ast::ParenPat`
#[pyclass(unsendable)]
pub struct AstParenPat {
    pub inner: ra_ap_syntax::ast::ParenPat,
}

/// `syntax::ast::ExternBlock`
#[pyclass(unsendable)]
pub struct AstExternBlock {
    pub inner: ra_ap_syntax::ast::ExternBlock,
}

/// `syntax::ast::ExternItemList`
#[pyclass(unsendable)]
pub struct AstExternItemList {
    pub inner: ra_ap_syntax::ast::ExternItemList,
}

/// `syntax::ast::ExternItem`
#[pyclass(unsendable)]
pub struct AstExternItem {
    pub inner: ra_ap_syntax::ast::ExternItem,
}

/// `syntax::ast::RecordExprField`
#[pyclass(unsendable)]
pub struct AstRecordExprField {
    pub inner: ra_ap_syntax::ast::RecordExprField,
}

/// `syntax::ast::ElseBranch`
#[pyclass(unsendable)]
pub struct AstElseBranch {
    pub inner: ra_ap_syntax::ast::ElseBranch,
}

/// `syntax::ast::ArrayExprKind`
#[pyclass(unsendable)]
pub struct AstArrayExprKind {
    pub inner: ra_ap_syntax::ast::ArrayExprKind,
}

/// `syntax::ast::BlockModifier`
#[pyclass(unsendable)]
pub struct AstBlockModifier {
    pub inner: ra_ap_syntax::ast::BlockModifier,
}

/// `syntax::ast::BinaryOp`
#[pyclass(unsendable)]
pub struct AstBinaryOp {
    pub inner: ra_ap_syntax::ast::BinaryOp,
}

/// `syntax::ast::FieldKind`
#[pyclass(unsendable)]
pub struct AstFieldKind {
    pub inner: ra_ap_syntax::ast::FieldKind,
}

/// `syntax::ast::NameOrNameRef`
#[pyclass(unsendable)]
pub struct AstNameOrNameRef {
    pub inner: ra_ap_syntax::ast::NameOrNameRef,
}

/// `syntax::ast::Type`
#[pyclass(unsendable)]
pub struct AstType {
    pub inner: ra_ap_syntax::ast::Type,
}

/// `syntax::ast::Label`
#[pyclass(unsendable)]
pub struct AstLabel {
    pub inner: ra_ap_syntax::ast::Label,
}

/// `hir::Const`
#[pyclass(unsendable)]
pub struct Const {
    pub inner: ra_ap_hir::Const,
}

/// `hir::Static`
#[pyclass(unsendable)]
pub struct Static {
    pub inner: ra_ap_hir::Static,
}

/// `hir::Macro`
#[pyclass(unsendable)]
pub struct Macro {
    pub inner: ra_ap_hir::Macro,
}

/// `hir::ConstParam`
#[pyclass(unsendable)]
pub struct ConstParam {
    pub inner: ra_ap_hir::ConstParam,
}

/// `hir::Local<'db>`, retaining the database that owns the local.
#[pyclass(unsendable)]
pub struct Local {
    pub inner: ra_ap_hir::Local<'static>,
    pub db: Py<Database>,
}

/// `hir::LocalSource<'db>`, retaining the database that owns the local.
#[pyclass(unsendable)]
pub struct LocalSource {
    pub inner: ra_ap_hir::LocalSource<'static>,
    pub _db: Py<Database>,
}

/// `hir::InFile<syntax::ast::Const>`
#[pyclass(unsendable)]
pub struct InFileAstConst {
    pub inner: ra_ap_hir::InFile<ra_ap_syntax::ast::Const>,
}

/// `hir::InFile<syntax::ast::Static>`
#[pyclass(unsendable)]
pub struct InFileAstStatic {
    pub inner: ra_ap_hir::InFile<ra_ap_syntax::ast::Static>,
}

/// `hir::InFile<syntax::ast::TypeAlias>`
#[pyclass(unsendable)]
pub struct InFileAstTypeAlias {
    pub inner: ra_ap_hir::InFile<ra_ap_syntax::ast::TypeAlias>,
}

/// `hir::InFile<either::Either<syntax::ast::Macro, syntax::ast::Fn>>`
#[pyclass(unsendable)]
pub struct InFileEitherAstMacroAstFn {
    pub inner: ra_ap_hir::InFile<either::Either<ra_ap_syntax::ast::Macro, ra_ap_syntax::ast::Fn>>,
}

/// `hir::InFile<either::Either<syntax::ast::TypeOrConstParam, syntax::ast::Trait>>`
#[pyclass(unsendable)]
pub struct InFileEitherAstTypeOrConstParamAstTrait {
    pub inner: ra_ap_hir::InFile<either::Either<ra_ap_syntax::ast::TypeOrConstParam, ra_ap_syntax::ast::Trait>>,
}

/// `hir::InFile<syntax::ast::Name>`
#[pyclass(unsendable)]
pub struct InFileAstName {
    pub inner: ra_ap_hir::InFile<ra_ap_syntax::ast::Name>,
}

/// `hir::InFile<&SyntaxNode>`, retaining the original syntax tree.
#[pyclass(unsendable)]
pub struct InFileSyntaxNodeRef {
    pub inner: ra_ap_hir::InFile<&'static ra_ap_syntax::SyntaxNode>,
    pub _owner: Py<PyAny>,
}

/// `syntax::ast::TypeAlias`
#[pyclass(unsendable)]
pub struct AstTypeAlias {
    pub inner: ra_ap_syntax::ast::TypeAlias,
}

/// `syntax::ast::Const`
#[pyclass(unsendable)]
pub struct AstConst {
    pub inner: ra_ap_syntax::ast::Const,
}

/// `syntax::ast::Static`
#[pyclass(unsendable)]
pub struct AstStatic {
    pub inner: ra_ap_syntax::ast::Static,
}

/// `syntax::ast::RangeExpr`
#[pyclass(unsendable)]
pub struct AstRangeExpr {
    pub inner: ra_ap_syntax::ast::RangeExpr,
}

/// `syntax::ast::RangePat`
#[pyclass(unsendable)]
pub struct AstRangePat {
    pub inner: ra_ap_syntax::ast::RangePat,
}

/// `hir_expand::InRealFile<SyntaxNode>`
#[pyclass(unsendable)]
pub struct InRealFileSyntaxNode {
    pub inner: ra_ap_hir_expand::InRealFile<ra_ap_syntax::SyntaxNode>,
}

/// `syntax::ast::String`
#[pyclass(unsendable)]
pub struct AstString {
    pub inner: ra_ap_syntax::ast::String,
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
    module.add_class::<HirDisplayWrapperTraitRef>()?;
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
    module.add_class::<AstArgList>()?;
    module.add_class::<AstAwaitExpr>()?;
    module.add_class::<AstBlockExpr>()?;
    module.add_class::<AstClosureExpr>()?;
    module.add_class::<AstExprStmt>()?;
    module.add_class::<AstExternCrate>()?;
    module.add_class::<AstItemList>()?;
    module.add_class::<AstLetExpr>()?;
    module.add_class::<AstMatchArmList>()?;
    module.add_class::<AstMatchExpr>()?;
    module.add_class::<AstMethodCallExpr>()?;
    module.add_class::<AstParenExpr>()?;
    module.add_class::<AstMacroRules>()?;
    module.add_class::<AstRefExpr>()?;
    module.add_class::<AstReturnExpr>()?;
    module.add_class::<AstTryExpr>()?;
    module.add_class::<AstUse>()?;
    module.add_class::<AstAssocItemList>()?;
    module.add_class::<AstAttr>()?;
    module.add_class::<AstCallExpr>()?;
    module.add_class::<AstExpr>()?;
    module.add_class::<AstFieldExpr>()?;
    module.add_class::<AstLetStmt>()?;
    module.add_class::<AstModule>()?;
    module.add_class::<AstRecordFieldList>()?;
    module.add_class::<AstRecordPat>()?;
    module.add_class::<AstStmtList>()?;
    module.add_class::<AstVariantList>()?;
    module.add_class::<AstForExpr>()?;
    module.add_class::<AstIfExpr>()?;
    module.add_class::<AstLoopExpr>()?;
    module.add_class::<AstPathExpr>()?;
    module.add_class::<AstWhileExpr>()?;
    module.add_class::<AstIdentPat>()?;
    module.add_class::<AstWildcardPat>()?;
    module.add_class::<AstStmt>()?;
    module.add_class::<AstPat>()?;
    module.add_class::<AstPath>()?;
    module.add_class::<AstCfgAttrMeta>()?;
    module.add_class::<AstKeyValueMeta>()?;
    module.add_class::<AstPathMeta>()?;
    module.add_class::<AstTokenTreeMeta>()?;
    module.add_class::<AstMeta>()?;
    module.add_class::<AstTokenTree>()?;
    module.add_class::<AstCfgPredicate>()?;
    module.add_class::<AstName>()?;
    module.add_class::<AstComment>()?;
    module.add_class::<AstAssocItem>()?;
    module.add_class::<AstRecordField>()?;
    module.add_class::<AstVariant>()?;
    module.add_class::<AstDocComment>()?;
    module.add_class::<AstRecordPatFieldList>()?;
    module.add_class::<AstRecordPatField>()?;
    module.add_class::<AstRestPat>()?;
    module.add_class::<EnumVariant>()?;
    module.add_class::<StructKind>()?;
    module.add_class::<Edition>()?;
    module.add_class::<CrateDisplayName>()?;
    module.add_class::<Impl>()?;
    module.add_class::<InFileAstModule>()?;
    module.add_class::<InFileModuleSource>()?;
    module.add_class::<AssocItem>()?;
    module.add_class::<Variant>()?;
    module.add_class::<TypeAlias>()?;
    module.add_class::<EitherFieldTupleField>()?;
    module.add_class::<TraitRef>()?;
    module.add_class::<Visibility>()?;
    module.add_class::<Target>()?;
    module.add_class::<TargetData>()?;
    module.add_class::<ManifestPath>()?;
    module.add_class::<TargetKind>()?;
    module.add_class::<BuiltinType>()?;
    module.add_class::<PathResolution>()?;
    module.add_class::<CargoFeatures>()?;
    module.add_class::<CfgOverrides>()?;
    module.add_class::<CfgDiff>()?;
    module.add_class::<CfgAtom>()?;
    module.add_class::<CfgExpr>()?;
    module.add_class::<CfgOptions>()?;
    module.add_class::<Symbol>()?;
    module.add_class::<TargetDirectoryConfig>()?;
    module.add_class::<Sysroot>()?;
    module.add_class::<Union>()?;
    module.add_class::<AstImpl>()?;
    module.add_class::<AstTrait>()?;
    module.add_class::<AstTupleField>()?;
    module.add_class::<AstNameRef>()?;
    module.add_class::<AstCfgMeta>()?;
    module.add_class::<InFileAstImpl>()?;
    module.add_class::<InFileAstTrait>()?;
    module.add_class::<InFileAstVariant>()?;
    module.add_class::<InFileFieldSource>()?;
    module.add_class::<FieldSource>()?;
    module.add_class::<TypeInfo>()?;
    module.add_class::<Docs>()?;
    module.add_class::<WideEncoding>()?;
    module.add_class::<WideLineCol>()?;
    module.add_class::<GenericParam>()?;
    module.add_class::<SyntaxError>()?;
    module.add_class::<crate::iter::Iter>()?;
    module.add_class::<AstParamList>()?;
    module.add_class::<AstParam>()?;
    module.add_class::<AstMatchArm>()?;
    module.add_class::<AstMatchGuard>()?;
    module.add_class::<AstLetElse>()?;
    module.add_class::<AstBinExpr>()?;
    module.add_class::<AstArrayExpr>()?;
    module.add_class::<AstTupleStructPat>()?;
    module.add_class::<AstTuplePat>()?;
    module.add_class::<AstSlicePat>()?;
    module.add_class::<AstOrPat>()?;
    module.add_class::<AstRefPat>()?;
    module.add_class::<AstParenPat>()?;
    module.add_class::<AstExternBlock>()?;
    module.add_class::<AstExternItemList>()?;
    module.add_class::<AstExternItem>()?;
    module.add_class::<AstRecordExprField>()?;
    module.add_class::<AstElseBranch>()?;
    module.add_class::<AstArrayExprKind>()?;
    module.add_class::<AstBlockModifier>()?;
    module.add_class::<AstBinaryOp>()?;
    module.add_class::<AstFieldKind>()?;
    module.add_class::<AstNameOrNameRef>()?;
    module.add_class::<AstType>()?;
    module.add_class::<AstLabel>()?;
    module.add_class::<Const>()?;
    module.add_class::<Static>()?;
    module.add_class::<Macro>()?;
    module.add_class::<ConstParam>()?;
    module.add_class::<Local>()?;
    module.add_class::<LocalSource>()?;
    module.add_class::<InFileAstConst>()?;
    module.add_class::<InFileAstStatic>()?;
    module.add_class::<InFileAstTypeAlias>()?;
    module.add_class::<InFileEitherAstMacroAstFn>()?;
    module.add_class::<InFileEitherAstTypeOrConstParamAstTrait>()?;
    module.add_class::<InFileAstName>()?;
    module.add_class::<InFileSyntaxNodeRef>()?;
    module.add_class::<AstTypeAlias>()?;
    module.add_class::<AstConst>()?;
    module.add_class::<AstStatic>()?;
    module.add_class::<AstRangeExpr>()?;
    module.add_class::<AstRangePat>()?;
    module.add_class::<InRealFileSyntaxNode>()?;
    module.add_class::<AstString>()?;
    Ok(())
}
