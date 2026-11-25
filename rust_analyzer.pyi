"""The types of the module `rust_analyzer`: one class for each type of rust-analyzer that a binding takes or
returns, with the bindings as its methods."""

from collections.abc import Callable as _Callable
from collections.abc import Iterator
from typing import Self, final, overload

@final
class AbsPath:
    """`paths::AbsPath`"""

    def __str__(self) -> str:
        """`impl Display for AbsPath`"""

@final
class AbsPathBuf:
    """`paths::AbsPathBuf`"""

    def deref(self) -> AbsPath:
        """`impl Deref<Target = AbsPath> for AbsPathBuf`, lent by this `AbsPathBuf`."""

    @staticmethod
    def try_from_str(value: str) -> AbsPathBuf:
        """`impl TryFrom<&str> for AbsPathBuf`: `fn try_from(path: &str) -> Result<AbsPathBuf, Utf8PathBuf>`"""

@final
class Adt:
    """`hir::Adt`"""

    def visibility(self, db: RootDatabase) -> Visibility:
        """`HasVisibility::visibility(&self, db: &dyn HirDatabase) -> Visibility`"""

    def hir_docs(self, db: RootDatabase) -> Docs | None:
        """`HasAttrs::hir_docs(self, db: &dyn HirDatabase) -> Option<&Docs>`, as a copy."""

    def as_enum(self) -> Enum | None:
        """`pub fn as_enum(&self) -> Option<Enum>`"""

    def as_struct(self) -> Struct | None:
        """`pub fn as_struct(&self) -> Option<Struct>`"""

    @staticmethod
    def Enum(value: Enum) -> Adt:
        """The variant `Adt::Enum(Enum)`, made."""

    def __eq__(self, other: object, /) -> bool:
        """`impl PartialEq for Adt`"""

    def __hash__(self) -> int:
        """`impl Hash for Adt`"""

    def module(self, db: RootDatabase) -> Module:
        """`pub fn module(self, db: &dyn HirDatabase) -> Module`"""

    def name(self, db: RootDatabase) -> Name:
        """`pub fn name(self, db: &dyn HirDatabase) -> Name`"""

    def source(self, db: RootDatabase) -> InFileAstAdt | None:
        """`impl HasSource for Adt`: `fn source(self, db: &dyn HirDatabase) -> Option<InFile<ast::Adt>>`"""

    def ty(self, db: RootDatabase) -> Type:
        """`pub fn ty(self, db: &'db dyn HirDatabase) -> Type<'db>`"""

    def as_Union(self) -> Union | None:
        """The variant `Adt::Union(Union)`, read: its `Union` if the value is that variant, otherwise `None`."""

    @staticmethod
    def Struct(value: Struct) -> Adt:
        """The variant `Adt::Struct(Struct)`, made."""

    @staticmethod
    def Union(value: Union) -> Adt:
        """The variant `Adt::Union(Union)`, made."""


@final
class AssocItem:
    """`hir::AssocItem`"""

    def as_Function(self) -> Function | None:
        """The variant `AssocItem::Function(Function)`, read: its `Function` if the value is that variant, otherwise `None`."""

    def as_type_alias(self) -> TypeAlias | None:
        """`pub fn as_type_alias(self) -> Option<TypeAlias>`"""


@final
class AstAdt:
    """`syntax::ast::Adt`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstAdt | None:
        """`impl AstNode for Adt`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is given a copy."""

    def syntax(self) -> SyntaxNode:
        """`impl AstNode for Adt`: `fn syntax(&self) -> &SyntaxNode`, lent by this `Adt`."""

    def name(self) -> AstName | None:
        """`HasName::name`: `fn name(&self) -> Option<ast::Name>`"""

@final
class AstAnyHasAttrs:
    """`syntax::ast::AnyHasAttrs`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstAnyHasAttrs | None:
        """`impl AstNode for AnyHasAttrs`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is given a copy."""

@final
class AstArgList:
    """`syntax::ast::ArgList`"""

    @staticmethod
    def can_cast(kind: SyntaxKind) -> bool:
        """`impl AstNode for ArgList`: `fn can_cast(kind: SyntaxKind) -> bool`"""

    def args(self) -> Iterator[AstExpr]:
        """`fn args(&self) -> AstChildren<Expr>`"""


@final
class AstAssocItem:
    """`syntax::ast::AssocItem`"""

    def syntax(self) -> SyntaxNode:
        """`impl AstNode for AssocItem`: `fn syntax(&self) -> &SyntaxNode`, lent by this `AssocItem`."""

@final
class AstAssocItemList:
    """`syntax::ast::AssocItemList`"""

    def assoc_items(self) -> Iterator[AstAssocItem]:
        """`pub fn assoc_items(&self) -> AstChildren<AssocItem>`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstAssocItemList | None:
        """`impl AstNode for AssocItemList`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is given a copy."""

@final
class AstAttr:
    """`syntax::ast::Attr`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstAttr | None:
        """`impl AstNode for Attr`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is given a copy."""

    def meta(self) -> AstMeta | None:
        """`pub fn meta(&self) -> Option<Meta>`"""

    def syntax(self) -> SyntaxNode:
        """`impl AstNode for Attr`: `fn syntax(&self) -> &SyntaxNode`, lent by this `Attr`."""

@final
class AstAwaitExpr:
    """`syntax::ast::AwaitExpr`"""

    @staticmethod
    def can_cast(kind: SyntaxKind) -> bool:
        """`impl AstNode for AwaitExpr`: `fn can_cast(kind: SyntaxKind) -> bool`"""

    def expr(self) -> AstExpr | None:
        """`pub fn expr(&self) -> Option<Expr>`"""

@final
class AstBlockExpr:
    """`syntax::ast::BlockExpr`"""

    def async_token(self) -> SyntaxToken | None:
        """`pub fn async_token(&self) -> Option<SyntaxToken>`"""

    @staticmethod
    def can_cast(kind: SyntaxKind) -> bool:
        """`impl AstNode for BlockExpr`: `fn can_cast(kind: SyntaxKind) -> bool`"""

    def stmt_list(self) -> AstStmtList | None:
        """`pub fn stmt_list(&self) -> Option<StmtList>`"""

    def syntax(self) -> SyntaxNode:
        """`impl AstNode for BlockExpr`: `fn syntax(&self) -> &SyntaxNode`, lent by this `BlockExpr`."""

    def modifier(self) -> AstBlockModifier | None:
        """`fn modifier(&self) -> Option<BlockModifier>`"""


@final
class AstCallExpr:
    """`syntax::ast::CallExpr`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstCallExpr | None:
        """`impl AstNode for CallExpr`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is given a copy."""

    def expr(self) -> AstExpr | None:
        """`pub fn expr(&self) -> Option<Expr>`"""

    def arg_list(self) -> AstArgList | None:
        """`fn arg_list(&self) -> Option<ArgList>`"""


@final
class AstCfgAttrMeta:
    """`syntax::ast::CfgAttrMeta`"""

    def cfg_predicate(self) -> AstCfgPredicate | None:
        """`pub fn cfg_predicate(&self) -> Option<CfgPredicate>`"""

    def metas(self) -> Iterator[AstMeta]:
        """`pub fn metas(&self) -> AstChildren<Meta>`"""

@final
class AstCfgMeta:
    """`syntax::ast::CfgMeta`"""

    def cfg_predicate(self) -> AstCfgPredicate | None:
        """`pub fn cfg_predicate(&self) -> Option<CfgPredicate>`"""

@final
class AstCfgPredicate:
    """`syntax::ast::CfgPredicate`"""

    def syntax(self) -> SyntaxNode:
        """`impl AstNode for CfgPredicate`: `fn syntax(&self) -> &SyntaxNode`, lent by this `CfgPredicate`."""

@final
class AstClosureExpr:
    """`syntax::ast::ClosureExpr`"""

    @staticmethod
    def can_cast(kind: SyntaxKind) -> bool:
        """`impl AstNode for ClosureExpr`: `fn can_cast(kind: SyntaxKind) -> bool`"""

    def param_list(self) -> AstParamList | None:
        """`fn param_list(&self) -> Option<ParamList>`"""

    def body(self) -> AstExpr | None:
        """`fn body(&self) -> Option<Expr>`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstClosureExpr | None:
        """`AstNode::cast(syntax: SyntaxNode) -> Option<ClosureExpr>`. It takes a copy of the node."""


@final
class AstComment:
    """`syntax::ast::Comment`"""

    def text(self) -> str:
        """`AstToken::text`: `fn text(&self) -> &str`"""

    @staticmethod
    def cast(syntax: SyntaxToken) -> AstComment | None:
        """`impl AstToken for Comment`: `fn cast(syntax: SyntaxToken) -> Option<Self>`. It takes the token, so it is given a copy."""

    def syntax(self) -> SyntaxToken:
        """`impl AstToken for Comment`: `fn syntax(&self) -> &SyntaxToken`, lent by this `Comment`."""

@final
class AstDocComment:
    """`syntax::ast::DocComment`"""

    def text(self) -> str:
        """`fn text(&self) -> &str`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstDocComment | None:
        """`AstNode::cast(syntax: SyntaxNode) -> Option<DocComment>`. It takes a copy."""

    @staticmethod
    def can_cast(kind: SyntaxKind) -> bool:
        """`impl AstNode for DocComment`: `fn can_cast(kind: SyntaxKind) -> bool`"""

@final
class AstExpr:
    """`syntax::ast::Expr`"""

    def as_AwaitExpr(self) -> AstAwaitExpr | None:
        """The variant `Expr::AwaitExpr(AwaitExpr)`, read: its `AwaitExpr`, lent by this `Expr`, if the value is that variant, otherwise `None`."""

    def as_BlockExpr(self) -> AstBlockExpr | None:
        """The variant `Expr::BlockExpr(BlockExpr)`, read: its `BlockExpr`, lent by this `Expr`, if the value is that variant, otherwise `None`."""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstExpr | None:
        """`impl AstNode for Expr`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is given a copy."""

    def as_ForExpr(self) -> AstForExpr | None:
        """The variant `Expr::ForExpr(ForExpr)`, read: its `ForExpr`, lent by this `Expr`, if the value is that variant, otherwise `None`."""

    def as_IfExpr(self) -> AstIfExpr | None:
        """The variant `Expr::IfExpr(IfExpr)`, read: its `IfExpr`, lent by this `Expr`, if the value is that variant, otherwise `None`."""

    def as_LoopExpr(self) -> AstLoopExpr | None:
        """The variant `Expr::LoopExpr(LoopExpr)`, read: its `LoopExpr`, lent by this `Expr`, if the value is that variant, otherwise `None`."""

    def as_MatchExpr(self) -> AstMatchExpr | None:
        """The variant `Expr::MatchExpr(MatchExpr)`, read: its `MatchExpr`, lent by this `Expr`, if the value is that variant, otherwise `None`."""

    def as_ParenExpr(self) -> AstParenExpr | None:
        """The variant `Expr::ParenExpr(ParenExpr)`, read: its `ParenExpr`, lent by this `Expr`, if the value is that variant, otherwise `None`."""

    def as_PathExpr(self) -> AstPathExpr | None:
        """The variant `Expr::PathExpr(PathExpr)`, read: its `PathExpr`, lent by this `Expr`, if the value is that variant, otherwise `None`."""

    def syntax(self) -> SyntaxNode:
        """`impl AstNode for Expr`: `fn syntax(&self) -> &SyntaxNode`, lent by this `Expr`."""

    def as_TryExpr(self) -> AstTryExpr | None:
        """The variant `Expr::TryExpr(TryExpr)`, read: its `TryExpr`, lent by this `Expr`, if the value is that variant, otherwise `None`."""

    def as_WhileExpr(self) -> AstWhileExpr | None:
        """The variant `Expr::WhileExpr(WhileExpr)`, read: its `WhileExpr`, lent by this `Expr`, if the value is that variant, otherwise `None`."""

@final
class AstExprStmt:
    """`syntax::ast::ExprStmt`"""

    @staticmethod
    def can_cast(kind: SyntaxKind) -> bool:
        """`impl AstNode for ExprStmt`: `fn can_cast(kind: SyntaxKind) -> bool`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstExprStmt | None:
        """`impl AstNode for ExprStmt`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is given a copy."""

    def expr(self) -> AstExpr | None:
        """`pub fn expr(&self) -> Option<Expr>`"""

@final
class AstExternCrate:
    """`syntax::ast::ExternCrate`"""

    @staticmethod
    def can_cast(kind: SyntaxKind) -> bool:
        """`impl AstNode for ExternCrate`: `fn can_cast(kind: SyntaxKind) -> bool`"""

@final
class AstFieldExpr:
    """`syntax::ast::FieldExpr`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstFieldExpr | None:
        """`impl AstNode for FieldExpr`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is given a copy."""

    def expr(self) -> AstExpr | None:
        """`pub fn expr(&self) -> Option<Expr>`"""

    def field_access(self) -> AstFieldKind | None:
        """`fn field_access(&self) -> Option<FieldKind>`"""


@final
class AstFn:
    """`syntax::ast::Fn`"""

    def body(self) -> AstBlockExpr | None:
        """`pub fn body(&self) -> Option<BlockExpr>`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstFn | None:
        """`impl AstNode for Fn`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is given a copy."""

    def syntax(self) -> SyntaxNode:
        """`impl AstNode for Fn`: `fn syntax(&self) -> &SyntaxNode`, lent by this `Fn`."""

    def attrs(self) -> Iterator[AstAttr]:
        """`HasAttrs::attrs`: `fn attrs(&self) -> AstChildren<ast::Attr>`"""

    def name(self) -> AstName | None:
        """`HasName::name`: `fn name(&self) -> Option<ast::Name>`"""

    def param_list(self) -> AstParamList | None:
        """`fn param_list(&self) -> Option<ParamList>`"""


@final
class AstForExpr:
    """`syntax::ast::ForExpr`"""

    def pat(self) -> AstPat | None:
        """`fn pat(&self) -> Option<Pat>`"""

    def iterable(self) -> AstExpr | None:
        """`fn iterable(&self) -> Option<Expr>`"""

    def loop_body(self) -> AstBlockExpr | None:
        """`fn loop_body(&self) -> Option<BlockExpr>`"""


@final
class AstIdentPat:
    """`syntax::ast::IdentPat`"""

    def pat(self) -> AstPat | None:
        """`pub fn pat(&self) -> Option<Pat>`"""

@final
class AstIfExpr:
    """`syntax::ast::IfExpr`"""

    def condition(self) -> AstExpr | None:
        """`fn condition(&self) -> Option<Expr>`"""

    def then_branch(self) -> AstBlockExpr | None:
        """`fn then_branch(&self) -> Option<BlockExpr>`"""

    def else_branch(self) -> AstElseBranch | None:
        """`fn else_branch(&self) -> Option<ElseBranch>`"""


@final
class AstImpl:
    """`syntax::ast::Impl`"""

    def assoc_item_list(self) -> AstAssocItemList | None:
        """`pub fn assoc_item_list(&self) -> Option<AssocItemList>`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstImpl | None:
        """`impl AstNode for Impl`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is given a copy."""

    def syntax(self) -> SyntaxNode:
        """`impl AstNode for Impl`: `fn syntax(&self) -> &SyntaxNode`, lent by this `Impl`."""

@final
class AstItem:
    """`syntax::ast::Item`"""

    @staticmethod
    def can_cast(kind: SyntaxKind) -> bool:
        """`impl AstNode for Item`: `fn can_cast(kind: SyntaxKind) -> bool`"""

    def syntax(self) -> SyntaxNode:
        """`impl AstNode for Item`: `fn syntax(&self) -> &SyntaxNode`, lent by this `Item`."""

@final
class AstItemList:
    """`syntax::ast::ItemList`"""

    def items(self) -> Iterator[AstItem]:
        """`impl HasModuleItem for ItemList`: `fn items(&self) -> AstChildren<ast::Item>`"""

    @staticmethod
    def can_cast(kind: SyntaxKind) -> bool:
        """`impl AstNode for ItemList`: `fn can_cast(kind: SyntaxKind) -> bool`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstItemList | None:
        """`impl AstNode for ItemList`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is given a copy."""

@final
class AstKeyValueMeta:
    """`syntax::ast::KeyValueMeta`"""

    def path(self) -> AstPath | None:
        """`pub fn path(&self) -> Option<Path>`"""

@final
class AstLetExpr:
    """`syntax::ast::LetExpr`"""

    @staticmethod
    def can_cast(kind: SyntaxKind) -> bool:
        """`impl AstNode for LetExpr`: `fn can_cast(kind: SyntaxKind) -> bool`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstLetExpr | None:
        """`impl AstNode for LetExpr`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is given a copy."""

    def expr(self) -> AstExpr | None:
        """`pub fn expr(&self) -> Option<Expr>`"""

    def pat(self) -> AstPat | None:
        """`pub fn pat(&self) -> Option<Pat>`"""

@final
class AstLetStmt:
    """`syntax::ast::LetStmt`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstLetStmt | None:
        """`impl AstNode for LetStmt`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is given a copy."""

    def initializer(self) -> AstExpr | None:
        """`pub fn initializer(&self) -> Option<Expr>`"""

    def pat(self) -> AstPat | None:
        """`pub fn pat(&self) -> Option<Pat>`"""

    def let_else(self) -> AstLetElse | None:
        """`fn let_else(&self) -> Option<LetElse>`"""


@final
class AstLoopExpr:
    """`syntax::ast::LoopExpr`"""

    def loop_body(self) -> AstBlockExpr | None:
        """`fn loop_body(&self) -> Option<BlockExpr>`"""


@final
class AstMacroCall:
    """`syntax::ast::MacroCall`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstMacroCall | None:
        """`impl AstNode for MacroCall`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is given a copy."""

    def syntax(self) -> SyntaxNode:
        """`impl AstNode for MacroCall`: `fn syntax(&self) -> &SyntaxNode`, lent by this `MacroCall`."""

@final
class AstMacroRules:
    """`syntax::ast::MacroRules`"""

    @staticmethod
    def can_cast(kind: SyntaxKind) -> bool:
        """`impl AstNode for MacroRules`: `fn can_cast(kind: SyntaxKind) -> bool`"""

@final
class AstMatchArmList:
    """`syntax::ast::MatchArmList`"""

    @staticmethod
    def can_cast(kind: SyntaxKind) -> bool:
        """`impl AstNode for MatchArmList`: `fn can_cast(kind: SyntaxKind) -> bool`"""

    def arms(self) -> Iterator[AstMatchArm]:
        """`fn arms(&self) -> AstChildren<MatchArm>`"""


@final
class AstMatchExpr:
    """`syntax::ast::MatchExpr`"""

    @staticmethod
    def can_cast(kind: SyntaxKind) -> bool:
        """`impl AstNode for MatchExpr`: `fn can_cast(kind: SyntaxKind) -> bool`"""

    def expr(self) -> AstExpr | None:
        """`pub fn expr(&self) -> Option<Expr>`"""

    def match_arm_list(self) -> AstMatchArmList | None:
        """`fn match_arm_list(&self) -> Option<MatchArmList>`"""


@final
class AstMeta:
    """`syntax::ast::Meta`"""

    def as_CfgAttrMeta(self) -> AstCfgAttrMeta | None:
        """The variant `Meta::CfgAttrMeta(CfgAttrMeta)`, read: its `CfgAttrMeta`, lent by this `Meta`, if the value is that variant, otherwise `None`."""

    def as_CfgMeta(self) -> AstCfgMeta | None:
        """The variant `Meta::CfgMeta(CfgMeta)`, read: its `CfgMeta`, lent by this `Meta`, if the value is that variant, otherwise `None`."""

    def as_KeyValueMeta(self) -> AstKeyValueMeta | None:
        """The variant `Meta::KeyValueMeta(KeyValueMeta)`, read: its `KeyValueMeta`, lent by this `Meta`, if the value is that variant, otherwise `None`."""

    def as_PathMeta(self) -> AstPathMeta | None:
        """The variant `Meta::PathMeta(PathMeta)`, read: its `PathMeta`, lent by this `Meta`, if the value is that variant, otherwise `None`."""

    def syntax(self) -> SyntaxNode:
        """`impl AstNode for Meta`: `fn syntax(&self) -> &SyntaxNode`, lent by this `Meta`."""

    def as_TokenTreeMeta(self) -> AstTokenTreeMeta | None:
        """The variant `Meta::TokenTreeMeta(TokenTreeMeta)`, read: its `TokenTreeMeta`, lent by this `Meta`, if the value is that variant, otherwise `None`."""

@final
class AstMethodCallExpr:
    """`syntax::ast::MethodCallExpr`"""

    @staticmethod
    def can_cast(kind: SyntaxKind) -> bool:
        """`impl AstNode for MethodCallExpr`: `fn can_cast(kind: SyntaxKind) -> bool`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstMethodCallExpr | None:
        """`impl AstNode for MethodCallExpr`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is given a copy."""

    def name_ref(self) -> AstNameRef | None:
        """`pub fn name_ref(&self) -> Option<NameRef>`"""

    def receiver(self) -> AstExpr | None:
        """`fn receiver(&self) -> Option<Expr>`"""

    def arg_list(self) -> AstArgList | None:
        """`fn arg_list(&self) -> Option<ArgList>`"""


@final
class AstModule:
    """`syntax::ast::Module`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstModule | None:
        """`impl AstNode for Module`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is given a copy."""

    def item_list(self) -> AstItemList | None:
        """`pub fn item_list(&self) -> Option<ItemList>`"""

    def syntax(self) -> SyntaxNode:
        """`impl AstNode for Module`: `fn syntax(&self) -> &SyntaxNode`, lent by this `Module`."""

@final
class AstName:
    """`syntax::ast::Name`"""

    def text(self) -> str:
        """`pub fn text(&self) -> &str`"""

    def syntax(self) -> SyntaxNode:
        """`impl AstNode for Name`: `fn syntax(&self) -> &SyntaxNode`, lent by this `Name`."""

@final
class AstNameRef:
    """`syntax::ast::NameRef`"""

    def as_name(self) -> Name:
        """`AsName::as_name(&self) -> Name`."""

    def syntax(self) -> SyntaxNode:
        """`impl AstNode for NameRef`: `fn syntax(&self) -> &SyntaxNode`, lent by this `NameRef`."""

@final
class AstParenExpr:
    """`syntax::ast::ParenExpr`"""

    @staticmethod
    def can_cast(kind: SyntaxKind) -> bool:
        """`impl AstNode for ParenExpr`: `fn can_cast(kind: SyntaxKind) -> bool`"""

    def expr(self) -> AstExpr | None:
        """`pub fn expr(&self) -> Option<Expr>`"""

@final
class AstPat:
    """`syntax::ast::Pat`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstPat | None:
        """`impl AstNode for Pat`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is given a copy."""

    def as_IdentPat(self) -> AstIdentPat | None:
        """The variant `Pat::IdentPat(IdentPat)`, read: its `IdentPat`, lent by this `Pat`, if the value is that variant, otherwise `None`."""

    def as_WildcardPat(self) -> AstWildcardPat | None:
        """The variant `Pat::WildcardPat(WildcardPat)`, read: its `WildcardPat`, lent by this `Pat`, if the value is that variant, otherwise `None`."""

    def syntax(self) -> SyntaxNode:
        """`fn syntax(&self) -> &SyntaxNode`, lent by the receiver."""


@final
class AstPath:
    """`syntax::ast::Path`"""

    def syntax(self) -> SyntaxNode:
        """`impl AstNode for Path`: `fn syntax(&self) -> &SyntaxNode`, lent by this `Path`."""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstPath | None:
        """`AstNode::cast(syntax: SyntaxNode) -> Option<Path>`. It takes a copy of the node."""


@final
class AstPathExpr:
    """`syntax::ast::PathExpr`"""

    def path(self) -> AstPath | None:
        """`pub fn path(&self) -> Option<Path>`"""

@final
class AstPathMeta:
    """`syntax::ast::PathMeta`"""

    def path(self) -> AstPath | None:
        """`pub fn path(&self) -> Option<Path>`"""

@final
class AstRecordField:
    """`syntax::ast::RecordField`"""

    def doc_comments(self) -> Iterator[AstDocComment]:
        """`HasAttrs::doc_comments`: `fn doc_comments(&self) -> AstChildren<ast::DocComment>`"""

    def syntax(self) -> SyntaxNode:
        """`impl AstNode for RecordField`: `fn syntax(&self) -> &SyntaxNode`, lent by this `RecordField`."""

@final
class AstRecordFieldList:
    """`syntax::ast::RecordFieldList`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstRecordFieldList | None:
        """`impl AstNode for RecordFieldList`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is given a copy."""

    def fields(self) -> Iterator[AstRecordField]:
        """`pub fn fields(&self) -> AstChildren<RecordField>`"""

@final
class AstRecordPat:
    """`syntax::ast::RecordPat`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstRecordPat | None:
        """`impl AstNode for RecordPat`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is given a copy."""

    def path(self) -> AstPath | None:
        """`pub fn path(&self) -> Option<Path>`"""

    def record_pat_field_list(self) -> AstRecordPatFieldList | None:
        """`pub fn record_pat_field_list(&self) -> Option<RecordPatFieldList>`"""

@final
class AstRecordPatField:
    """`syntax::ast::RecordPatField`"""

    def pat(self) -> AstPat | None:
        """`pub fn pat(&self) -> Option<Pat>`"""

    def field_name(self) -> AstNameOrNameRef | None:
        """`fn field_name(&self) -> Option<NameOrNameRef>`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstRecordPatField | None:
        """`AstNode::cast(syntax: SyntaxNode) -> Option<RecordPatField>`. It takes a copy of the node."""


@final
class AstRecordPatFieldList:
    """`syntax::ast::RecordPatFieldList`"""

    def fields(self) -> Iterator[AstRecordPatField]:
        """`pub fn fields(&self) -> AstChildren<RecordPatField>`"""

    def rest_pat(self) -> AstRestPat | None:
        """`pub fn rest_pat(&self) -> Option<RestPat>`"""

@final
class AstRefExpr:
    """`syntax::ast::RefExpr`"""

    @staticmethod
    def can_cast(kind: SyntaxKind) -> bool:
        """`impl AstNode for RefExpr`: `fn can_cast(kind: SyntaxKind) -> bool`"""

@final
class AstRestPat:
    """`syntax::ast::RestPat`"""

@final
class AstReturnExpr:
    """`syntax::ast::ReturnExpr`"""

    @staticmethod
    def can_cast(kind: SyntaxKind) -> bool:
        """`impl AstNode for ReturnExpr`: `fn can_cast(kind: SyntaxKind) -> bool`"""

    def expr(self) -> AstExpr | None:
        """`fn expr(&self) -> Option<Expr>`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstReturnExpr | None:
        """`AstNode::cast(syntax: SyntaxNode) -> Option<ReturnExpr>`. It takes a copy of the node."""


@final
class AstSourceFile:
    """`syntax::ast::SourceFile`"""

    def items(self) -> Iterator[AstItem]:
        """`impl HasModuleItem for SourceFile`: `fn items(&self) -> AstChildren<ast::Item>`. Python takes each item with `impl<N: AstNode> Iterator for AstChildren<N>`: `fn next(&mut self) -> Option<N>`."""

    @staticmethod
    def can_cast(kind: SyntaxKind) -> bool:
        """`impl AstNode for SourceFile`: `fn can_cast(kind: SyntaxKind) -> bool`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstSourceFile | None:
        """`impl AstNode for SourceFile`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is given a copy."""

    def syntax(self) -> SyntaxNode:
        """`impl AstNode for SourceFile`: `fn syntax(&self) -> &SyntaxNode`, lent by this `SourceFile`."""

@final
class AstStmt:
    """`syntax::ast::Stmt`"""

    def syntax(self) -> SyntaxNode:
        """`impl AstNode for Stmt`: `fn syntax(&self) -> &SyntaxNode`, lent by this `Stmt`."""

@final
class AstStmtList:
    """`syntax::ast::StmtList`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstStmtList | None:
        """`impl AstNode for StmtList`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is given a copy."""

    def statements(self) -> Iterator[AstStmt]:
        """`pub fn statements(&self) -> AstChildren<Stmt>`"""

    def tail_expr(self) -> AstExpr | None:
        """`pub fn tail_expr(&self) -> Option<Expr>`"""

@final
class AstTokenTree:
    """`syntax::ast::TokenTree`"""

    def syntax(self) -> SyntaxNode:
        """`impl AstNode for TokenTree`: `fn syntax(&self) -> &SyntaxNode`, lent by this `TokenTree`."""

@final
class AstTokenTreeMeta:
    """`syntax::ast::TokenTreeMeta`"""

    def path(self) -> AstPath | None:
        """`pub fn path(&self) -> Option<Path>`"""

    def token_tree(self) -> AstTokenTree | None:
        """`pub fn token_tree(&self) -> Option<TokenTree>`"""

@final
class AstTrait:
    """`syntax::ast::Trait`"""

    def assoc_item_list(self) -> AstAssocItemList | None:
        """`pub fn assoc_item_list(&self) -> Option<AssocItemList>`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstTrait | None:
        """`impl AstNode for Trait`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is given a copy."""

    def syntax(self) -> SyntaxNode:
        """`impl AstNode for Trait`: `fn syntax(&self) -> &SyntaxNode`, lent by this `Trait`."""

@final
class AstTryExpr:
    """`syntax::ast::TryExpr`"""

    @staticmethod
    def can_cast(kind: SyntaxKind) -> bool:
        """`impl AstNode for TryExpr`: `fn can_cast(kind: SyntaxKind) -> bool`"""

    def expr(self) -> AstExpr | None:
        """`pub fn expr(&self) -> Option<Expr>`"""

@final
class AstTupleField:
    """`syntax::ast::TupleField`"""

    def syntax(self) -> SyntaxNode:
        """`impl AstNode for TupleField`: `fn syntax(&self) -> &SyntaxNode`, lent by this `TupleField`."""

@final
class AstUse:
    """`syntax::ast::Use`"""

    @staticmethod
    def can_cast(kind: SyntaxKind) -> bool:
        """`impl AstNode for Use`: `fn can_cast(kind: SyntaxKind) -> bool`"""

@final
class AstVariant:
    """`syntax::ast::Variant`"""

    def doc_comments(self) -> Iterator[AstDocComment]:
        """`HasAttrs::doc_comments`: `fn doc_comments(&self) -> AstChildren<ast::DocComment>`"""

    def syntax(self) -> SyntaxNode:
        """`impl AstNode for Variant`: `fn syntax(&self) -> &SyntaxNode`, lent by this `Variant`."""

@final
class AstVariantList:
    """`syntax::ast::VariantList`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstVariantList | None:
        """`impl AstNode for VariantList`: `fn cast(syntax: SyntaxNode) -> Option<Self>`. It takes the node, so it is given a copy."""

    def variants(self) -> Iterator[AstVariant]:
        """`pub fn variants(&self) -> AstChildren<Variant>`"""

@final
class AstWhileExpr:
    """`syntax::ast::WhileExpr`"""

    def condition(self) -> AstExpr | None:
        """`fn condition(&self) -> Option<Expr>`"""

    def loop_body(self) -> AstBlockExpr | None:
        """`fn loop_body(&self) -> Option<BlockExpr>`"""


@final
class AstWildcardPat:
    """`syntax::ast::WildcardPat`"""

@final
class BuiltinType:
    """`hir::BuiltinType`"""

    def name(self) -> Name:
        """`pub fn name(self) -> Name`"""

@final
class Callable:
    """`hir::Callable<'db>`. It borrows the database, so it holds the database's Python object."""

    def params(self) -> list[Param]:
        """`pub fn params(&self) -> Vec<Param<'db>>`"""

    def return_type(self) -> Type:
        """`pub fn return_type(&self) -> Type<'db>`"""

@final
class CargoConfig:
    """`project_model::CargoConfig`"""

    @property
    def all_targets(self) -> bool:
        """`pub all_targets: bool`"""

    @all_targets.setter
    def all_targets(self, value: bool) -> None: ...

    @property
    def cfg_overrides(self) -> CfgOverrides:
        """`pub cfg_overrides: CfgOverrides`. Copied, because the setter changes the `CargoConfig`."""

    @cfg_overrides.setter
    def cfg_overrides(self, value: CfgOverrides) -> None: ...

    @staticmethod
    def default() -> CargoConfig:
        """`impl Default for CargoConfig`"""

    @property
    def extra_args(self) -> list[str]:
        """`pub extra_args: Vec<String>`. Copied, because the setter changes the `CargoConfig`."""

    @extra_args.setter
    def extra_args(self, value: list[str]) -> None: ...

    @property
    def extra_env(self) -> dict[str, str | None]:
        """`pub extra_env: FxHashMap<String, Option<String>>`. Copied, because the setter changes the `CargoConfig`."""

    @extra_env.setter
    def extra_env(self, value: dict[str, str | None]) -> None: ...

    @property
    def features(self) -> CargoFeatures:
        """`pub features: CargoFeatures`. Copied, because the setter changes the `CargoConfig`."""

    @features.setter
    def features(self, value: CargoFeatures) -> None: ...

    @property
    def set_test(self) -> bool:
        """`pub set_test: bool`"""

    @set_test.setter
    def set_test(self, value: bool) -> None: ...

    @property
    def sysroot(self) -> RustLibSource | None:
        """`pub sysroot: Option<RustLibSource>`. Copied, because the setter changes the `CargoConfig`."""

    @sysroot.setter
    def sysroot(self, value: RustLibSource | None) -> None: ...

    @property
    def target(self) -> str | None:
        """`pub target: Option<String>`. Copied, because the setter changes the `CargoConfig`."""

    @target.setter
    def target(self, value: str | None) -> None: ...

    @property
    def target_dir_config(self) -> TargetDirectoryConfig:
        """`pub target_dir_config: TargetDirectoryConfig`. Copied, because the setter changes the `CargoConfig`."""

    @target_dir_config.setter
    def target_dir_config(self, value: TargetDirectoryConfig) -> None: ...

@final
class CargoFeatures:
    """`project_model::CargoFeatures`"""

    @staticmethod
    def All() -> CargoFeatures:
        """The variant `CargoFeatures::All`, made."""

    @staticmethod
    def Selected(features: list[str], no_default_features: bool) -> CargoFeatures:
        """The variant `CargoFeatures::Selected { features: Vec<String>, no_default_features: bool }`, made."""

@final
class CargoWorkspace:
    """`project_model::CargoWorkspace`"""

    def clone(self) -> CargoWorkspace:
        """`impl Clone for CargoWorkspace`"""

    def index_Package(self, index: Package) -> PackageData:
        """`impl Index<Package> for CargoWorkspace`: `fn index(&self, index: Package) -> &PackageData`, lent by this `CargoWorkspace`."""

    def index_Target(self, index: Target) -> TargetData:
        """`impl Index<Target> for CargoWorkspace`: `fn index(&self, index: Target) -> &TargetData`, lent by this `CargoWorkspace`."""

    def packages(self) -> Iterator[Package]:
        """`pub fn packages(&self) -> impl ExactSizeIterator<Item = Package> + '_`, lent by this `CargoWorkspace`."""

    def target_by_root(self, root: AbsPath) -> Target | None:
        """`pub fn target_by_root(&self, root: &AbsPath) -> Option<Target>`"""

@final
class CfgAtom:
    """`cfg::CfgAtom`"""

    @staticmethod
    def Flag(symbol: Symbol) -> CfgAtom:
        """The variant `CfgAtom::Flag(Symbol)`, made. It takes the symbol, so it is given a copy."""

    @staticmethod
    def KeyValue(key: Symbol, value: Symbol) -> CfgAtom:
        """The variant `CfgAtom::KeyValue { key: Symbol, value: Symbol }`, made. It takes the symbols, so it is given copies."""

@final
class CfgDiff:
    """`cfg::CfgDiff`"""

    @staticmethod
    def new(enable: list[CfgAtom], disable: list[CfgAtom]) -> CfgDiff:
        """`pub fn new(enable: Vec<CfgAtom>, disable: Vec<CfgAtom>) -> CfgDiff`. It takes the atoms, so it is given copies."""

@final
class CfgExpr:
    """`cfg::CfgExpr`"""

    @staticmethod
    def parse_from_ast(ast: AstCfgPredicate) -> CfgExpr:
        """`pub fn parse_from_ast(ast: syntax::ast::CfgPredicate) -> CfgExpr`. It takes the predicate, so it is given a copy."""

@final
class CfgOptions:
    """`cfg::CfgOptions`"""

    def apply_diff(self, diff: CfgDiff) -> None:
        """`pub fn apply_diff(&mut self, diff: CfgDiff)`. It takes the diff, so it is given a copy."""

    def check(self, cfg: CfgExpr) -> bool | None:
        """`pub fn check(&self, cfg: &CfgExpr) -> Option<bool>`"""

@final
class CfgOverrides:
    """`project_model::CfgOverrides`"""

    def __new__(cls, global_: CfgDiff, selective: dict[str, CfgDiff], /) -> Self:
        """The struct literal: `global: cfg::CfgDiff` and `selective: FxHashMap<String, cfg::CfgDiff>`. It takes the diffs, so it is given copies."""

@final
class Crate:
    """`hir::Crate`"""

    @staticmethod
    def all(db: RootDatabase) -> list[Crate]:
        """`pub fn all(db: &dyn HirDatabase) -> Vec<Crate>`"""

    def cfg(self, db: RootDatabase) -> CfgOptions:
        """`pub fn cfg<'db>(&self, db: &'db dyn HirDatabase) -> &'db CfgOptions`. Copied, because `apply_diff` changes a `CfgOptions`."""

    def display_name(self, db: RootDatabase) -> CrateDisplayName | None:
        """`pub fn display_name(self, db: &dyn HirDatabase) -> Option<CrateDisplayName>`"""

    def edition(self, db: RootDatabase) -> Edition:
        """`pub fn edition(self, db: &dyn HirDatabase) -> Edition`"""

    def modules(self, db: RootDatabase) -> list[Module]:
        """`pub fn modules(self, db: &dyn HirDatabase) -> Vec<Module>`"""

    def root_file(self, db: RootDatabase) -> FileId:
        """`pub fn root_file(self, db: &dyn HirDatabase) -> FileId`"""

    def root_module(self, db: RootDatabase) -> Module:
        """`pub fn root_module(self, db: &dyn HirDatabase) -> Module`"""

    def to_display_target(self, db: RootDatabase) -> DisplayTarget:
        """`pub fn to_display_target(self, db: &dyn HirDatabase) -> DisplayTarget`"""

    def version(self, db: RootDatabase) -> str | None:
        """`pub fn version(self, db: &dyn HirDatabase) -> Option<String>`"""


@final
class CrateDisplayName:
    """`base_db::CrateDisplayName`"""

    def __str__(self) -> str:
        """`impl Display for CrateDisplayName`"""

@final
class DepKind:
    """`project_model::DepKind`. rust-analyzer does not export the type, so it cannot be named outside rust-analyzer; the class holds the `PackageDependency` whose `kind` it is."""

    def __repr__(self) -> str:
        """`impl Debug for DepKind`. The variants cannot be read, because the type cannot be named.  Python tells the kinds apart by this text (`repr(kind) == "Dev"`). That works because `DepKind` derives `Debug`, so the text is the variant's name: `Normal`, `Dev` or `Build`. If a rust-analyzer upgrade renamed a variant, every comparison with the old name would be false, and dev-dependencies would count as normal ones. `test_a_package_lends_its_dependencies` reads all three kinds and would fail then."""

@final
class DisplayTarget:
    """`hir::DisplayTarget`"""

@final
class Docs:
    """`hir::Docs`"""

    def docs(self) -> str:
        """`pub fn docs(&self) -> &str`"""

@final
class Edition:
    """`syntax::Edition`"""

@final
class EditionedFileId:
    """`hir::EditionedFileId`"""

    def file_id(self, db: RootDatabase) -> FileId:
        """`pub fn file_id(self, db: &dyn Database) -> vfs::FileId`"""

    def parse_errors(self, db: RootDatabase) -> list[SyntaxError] | None:
        """`pub fn parse_errors(self, db: &dyn SourceDatabase) -> Option<&[SyntaxError]>`, which the query returns by reference: the errors as a list of copies."""

@final
class Enum:
    """`hir::Enum`"""

    def variants(self, db: RootDatabase) -> list[EnumVariant]:
        """`pub fn variants(self, db: &dyn HirDatabase) -> Vec<EnumVariant>`"""

@final
class EnumVariant:
    """`hir::EnumVariant`"""

    def fields(self, db: RootDatabase) -> list[Field]:
        """`pub fn fields(self, db: &dyn HirDatabase) -> Vec<Field>`"""

    def kind(self, db: RootDatabase) -> StructKind:
        """`pub fn kind(self, db: &dyn HirDatabase) -> StructKind`"""

    def name(self, db: RootDatabase) -> Name:
        """`pub fn name(self, db: &dyn HirDatabase) -> Name`"""

    def parent_enum(self, db: RootDatabase) -> Enum:
        """`pub fn parent_enum(self, db: &dyn HirDatabase) -> Enum`"""

    def hir_docs(self, db: RootDatabase) -> Docs | None:
        """`HasAttrs::hir_docs`: `fn hir_docs(self, db: &dyn HirDatabase) -> Option<&Docs>`, as a copy."""

@final
class Field:
    """`hir::Field`"""

    def name(self, db: RootDatabase) -> Name:
        """`pub fn name(&self, db: &dyn HirDatabase) -> Name`"""

    def ty(self, db: RootDatabase) -> Type:
        """`pub fn ty<'db>(&self, db: &'db dyn HirDatabase) -> Type<'db>`"""

    def visibility(self, db: RootDatabase) -> Visibility:
        """`impl HasVisibility for Field`: `fn visibility(&self, db: &dyn HirDatabase) -> Visibility`"""

    def hir_docs(self, db: RootDatabase) -> Docs | None:
        """`HasAttrs::hir_docs`: `fn hir_docs(self, db: &dyn HirDatabase) -> Option<&Docs>`, as a copy."""

    def is_visible_from(self, db: RootDatabase, module: Module) -> bool:
        """`HasVisibility::is_visible_from`: `fn is_visible_from(&self, db: &dyn HirDatabase, module: Module) -> bool`"""

    def parent_def(self, db: RootDatabase) -> Variant:
        """`pub fn parent_def(&self, _db: &dyn HirDatabase) -> Variant`"""


@final
class FieldSource:
    """`hir::FieldSource`, lent by the `InFile` that holds it: the type cannot be copied."""

    def as_Named(self) -> AstRecordField | None:
        """The variant `FieldSource::Named(ast::RecordField)`, read: a copy of its `RecordField` if the value is that variant, otherwise `None`."""

    def as_Pos(self) -> AstTupleField | None:
        """The variant `FieldSource::Pos(ast::TupleField)`, read: a copy of its `TupleField` if the value is that variant, otherwise `None`."""

@final
class FileId:
    """`vfs::FileId`"""

@final
class FileRange:
    """`hir::FileRange`, which is `FileRangeWrapper<EditionedFileId>`"""

    @property
    def file_id(self) -> EditionedFileId:
        """`pub file_id: EditionedFileId` of `FileRangeWrapper<EditionedFileId>`"""

    @property
    def range(self) -> TextRange:
        """`pub range: TextRange`"""

@final
class Function:
    """`hir::Function`"""

    def visibility(self, db: RootDatabase) -> Visibility:
        """`HasVisibility::visibility(&self, db: &dyn HirDatabase) -> Visibility`"""

    def hir_docs(self, db: RootDatabase) -> Docs | None:
        """`HasAttrs::hir_docs(self, db: &dyn HirDatabase) -> Option<&Docs>`, as a copy."""

    def async_ret_type(self, db: RootDatabase) -> Type | None:
        """`pub fn async_ret_type<'db>(self, db: &'db dyn HirDatabase) -> Option<Type<'db>>`"""

    def __eq__(self, other: object, /) -> bool:
        """`impl PartialEq for Function`"""

    def __hash__(self) -> int:
        """`impl Hash for Function`"""

    def is_async(self, db: RootDatabase) -> bool:
        """`pub fn is_async(self, db: &dyn HirDatabase) -> bool`"""

    def is_ignore(self, db: RootDatabase) -> bool:
        """`pub fn is_ignore(self, db: &dyn HirDatabase) -> bool`"""

    def is_test(self, db: RootDatabase) -> bool:
        """`pub fn is_test(self, db: &dyn HirDatabase) -> bool`"""

    def name(self, db: RootDatabase) -> Name:
        """`pub fn name(self, db: &dyn HirDatabase) -> Name`"""

    def ret_type(self, db: RootDatabase) -> Type:
        """`pub fn ret_type(self, db: &dyn HirDatabase) -> Type<'_>`"""

    def source(self, db: RootDatabase) -> InFileAstFn | None:
        """`impl HasSource for Function`: `fn source(self, db: &dyn HirDatabase) -> Option<InFile<ast::Fn>>`"""

    def assoc_fn_params(self, db: RootDatabase) -> list[Param]:
        """`fn assoc_fn_params(self, db: &dyn HirDatabase) -> Vec<Param<'_>>`"""


@final
class GenericDef:
    """`hir::GenericDef`"""

    @staticmethod
    def from_Adt(adt: Adt) -> GenericDef:
        """`impl From<Adt> for GenericDef`: `fn from(value: Adt) -> Self`"""

    @staticmethod
    def from_Trait(value: Trait) -> GenericDef:
        """`impl From<Trait> for GenericDef`: `fn from(value: Trait) -> Self`"""

    def params(self, db: RootDatabase) -> list[GenericParam]:
        """`pub fn params(self, db: &dyn HirDatabase) -> Vec<GenericParam>`"""

    def type_or_const_params(self, db: RootDatabase) -> list[TypeOrConstParam]:
        """`pub fn type_or_const_params(self, db: &dyn HirDatabase) -> Vec<TypeOrConstParam>`"""

    @staticmethod
    def from_Function(function: Function) -> GenericDef:
        """`impl From<Function> for GenericDef`"""

    @staticmethod
    def from_Impl(impl_: Impl) -> GenericDef:
        """`impl From<Impl> for GenericDef`"""

    @staticmethod
    def from_TypeAlias(type_alias: TypeAlias) -> GenericDef:
        """`impl From<TypeAlias> for GenericDef`"""


@final
class GenericParam:
    """`hir::GenericParam`"""

@final
class HirDisplayWrapperType:
    """`hir_ty::display::HirDisplayWrapper<'a, 'db, Type<'db>>`. It borrows the type it displays and the database, so it holds the Python objects of both."""

    def __str__(self) -> str:
        """`impl Display for HirDisplayWrapper<'_, '_, T>`"""

@final
class HirDisplayWrapperTraitRef:
    """`hir_ty::display::HirDisplayWrapper<'a, 'db, TraitRef<'db>>`. It borrows the trait reference it displays and the database, so it holds the Python objects of both."""

    def __str__(self) -> str:
        """`impl Display for HirDisplayWrapper<'_, '_, T>`"""

@final
class HirFileId:
    """`hir::HirFileId`"""

    def original_call_node(self, db: RootDatabase) -> InRealFileSyntaxNode | None:
        """`fn original_call_node(self, db: &dyn SourceDatabase) -> Option<InRealFile<SyntaxNode>>`"""

    def file_id(self) -> EditionedFileId | None:
        """`pub fn file_id(self) -> Option<EditionedFileId>`"""

    def as_MacroFile(self) -> MacroCallId | None:
        """The variant `HirFileId::MacroFile(MacroCallId)`, read: its `MacroCallId` if the value is that variant, otherwise `None`."""

@final
class Impl:
    """`hir::Impl`"""

    def hir_docs(self, db: RootDatabase) -> Docs | None:
        """`HasAttrs::hir_docs(self, db: &dyn HirDatabase) -> Option<&Docs>`, as a copy."""

    @staticmethod
    def all_for_type(db: RootDatabase, ty: Type) -> list[Impl]:
        """`pub fn all_for_type<'db>(db: &'db dyn HirDatabase, ty: Type<'db>) -> Vec<Impl>`. It takes the type, so it is given a copy."""

    def items(self, db: RootDatabase) -> list[AssocItem]:
        """`pub fn items(self, db: &dyn HirDatabase) -> Vec<AssocItem>`"""

    def self_ty(self, db: RootDatabase) -> Type:
        """`pub fn self_ty(self, db: &dyn HirDatabase) -> Type<'_>`"""

    def trait_(self, db: RootDatabase) -> Trait | None:
        """`pub fn trait_(self, db: &dyn HirDatabase) -> Option<Trait>`"""

    def trait_ref(self, db: RootDatabase) -> TraitRef | None:
        """`pub fn trait_ref(self, db: &dyn HirDatabase) -> Option<TraitRef<'_>>`"""

@final
class InFileAstAdt:
    """`hir::InFile<ast::Adt>`"""

    @property
    def file_id(self) -> HirFileId:
        """`pub file_id: HirFileId`"""

    @property
    def value(self) -> AstAdt:
        """`pub value: ast::Adt`, lent by this `InFile`."""

@final
class InFileAstFn:
    """`hir::InFile<ast::Fn>`"""

    @property
    def file_id(self) -> HirFileId:
        """`pub file_id: HirFileId`"""

    @property
    def value(self) -> AstFn:
        """`pub value: ast::Fn`, lent by this `InFile`."""

@final
class InFileAstImpl:
    """`hir::InFile<ast::Impl>`"""

    @property
    def file_id(self) -> HirFileId:
        """`pub file_id: HirFileId`"""

    @property
    def value(self) -> AstImpl:
        """`pub value: T`, with `T` the `ast::Impl`, lent by this `InFile`."""

@final
class InFileAstModule:
    """`hir::InFile<ast::Module>`"""

    @property
    def file_id(self) -> HirFileId:
        """`pub file_id: HirFileId`"""

    @property
    def value(self) -> AstModule:
        """`pub value: T`, with `T` the `ast::Module`, lent by this `InFile`."""

@final
class InFileAstTrait:
    """`hir::InFile<ast::Trait>`"""

    @property
    def file_id(self) -> HirFileId:
        """`pub file_id: HirFileId`"""

    @property
    def value(self) -> AstTrait:
        """`pub value: T`, with `T` the `ast::Trait`, lent by this `InFile`."""

@final
class InFileAstVariant:
    """`hir::InFile<ast::Variant>`"""

    @property
    def file_id(self) -> HirFileId:
        """`pub file_id: HirFileId`"""

    @property
    def value(self) -> AstVariant:
        """`pub value: T`, with `T` the `ast::Variant`, as a copy."""

@final
class InFileFieldSource:
    """`hir::InFile<FieldSource>`"""

    @property
    def file_id(self) -> HirFileId:
        """`pub file_id: HirFileId`"""

    @property
    def value(self) -> FieldSource:
        """`pub value: T`, with `T` the `FieldSource`, lent by this `InFile`."""

@final
class InFileModuleSource:
    """`hir::InFile<ModuleSource>`"""

    @property
    def file_id(self) -> HirFileId:
        """`pub file_id: HirFileId`"""

@final
class InFileSmallVecTextRange:
    """`hir::InFile<SmallVec<[TextRange; 1]>>`"""

    @property
    def file_id(self) -> HirFileId:
        """`pub file_id: HirFileId`"""

    @property
    def value(self) -> list[TextRange]:
        """`pub value: SmallVec<[TextRange; 1]>`, as a list."""

@final
class InFileSyntaxNode:
    """`hir::InFile<SyntaxNode>`"""

    @property
    def file_id(self) -> HirFileId:
        """`pub file_id: HirFileId`"""

    @staticmethod
    def new(file_id: HirFileId, value: SyntaxNode) -> InFileSyntaxNode:
        """`pub fn new(file_id: FileKind, value: T) -> InFileWrapper<FileKind, T>`, with `FileKind` the `HirFileId` and `T` the `SyntaxNode`. It takes the node, so it is given a copy."""

    def original_file_range_rooted(self, db: RootDatabase) -> FileRange:
        """`impl<SN: Borrow<SyntaxNode>> InFile<SN>`: `pub fn original_file_range_rooted(self, db: &dyn SourceDatabase) -> FileRange`. It takes the value, so it is given a copy."""

    @property
    def value(self) -> SyntaxNode:
        """`pub value: SyntaxNode`, lent by this `InFile`."""

@final
class InFileTextRange:
    """`hir::InFile<TextRange>`"""

    @property
    def file_id(self) -> HirFileId:
        """`pub file_id: HirFileId`"""

    @staticmethod
    def new(file_id: HirFileId, value: TextRange) -> InFileTextRange:
        """`pub fn new(file_id: FileKind, value: T) -> InFileWrapper<FileKind, T>`, with `T` the `TextRange`."""

    def original_node_file_range_rooted(self, db: RootDatabase) -> FileRange:
        """`impl InFile<TextRange>`: `pub fn original_node_file_range_rooted(self, db: &dyn SourceDatabase) -> FileRange`"""

    @property
    def value(self) -> TextRange:
        """`pub value: TextRange`"""

@final
class InMacroFileSyntaxToken:
    """`hir_expand::InMacroFile<SyntaxToken>`"""

    @staticmethod
    def new(file_id: MacroCallId, value: SyntaxToken) -> InMacroFileSyntaxToken:
        """`pub fn new(file_id: FileKind, value: T) -> InFileWrapper<FileKind, T>`, with `FileKind` the `MacroCallId` and `T` the `SyntaxToken`. It takes the token, so it is given a copy."""

    def upmap_once(self, db: RootDatabase) -> InFileSmallVecTextRange:
        """`impl InMacroFile<SyntaxToken>`: `pub fn upmap_once(self, db: &dyn SourceDatabase) -> InFile<SmallVec<[TextRange; 1]>>`. It takes the value, so it is given a copy."""

@final
class Iter:
    """A Rust iterator, taken one item at a time from Python. It keeps alive the Python object the iterator borrows from, if any."""

    def __iter__(self) -> Self:
        """The iterator itself."""

    def __next__(self) -> object:
        """`fn next(&mut self) -> Option<Self::Item>`"""

@final
class LineCol:
    """`line_index::LineCol`"""

    @property
    def col(self) -> int:
        """`pub col: u32`"""

    @property
    def line(self) -> int:
        """`pub line: u32`"""

@final
class LineIndex:
    """`line_index::LineIndex`"""

    def line_col(self, offset: TextSize) -> LineCol:
        """`pub fn line_col(&self, offset: TextSize) -> LineCol`"""

    @staticmethod
    def new(text: str) -> LineIndex:
        """`pub fn new(text: &str) -> LineIndex`"""

    def to_wide(self, enc: WideEncoding, line_col: LineCol) -> WideLineCol | None:
        """`pub fn to_wide(&self, enc: WideEncoding, line_col: LineCol) -> Option<WideLineCol>`"""

@final
class LintAttr:
    """`hir::LintAttr`"""

    def as_Allow(self) -> bool:
        """The variant `LintAttr::Allow`, read: whether the value is that variant, which holds nothing."""

    def as_Expect(self) -> bool:
        """The variant `LintAttr::Expect`, read: whether the value is that variant, which holds nothing."""

@final
class LoadCargoConfig:
    """`load_cargo::LoadCargoConfig`"""

    @property
    def load_out_dirs_from_check(self) -> bool:
        """`pub load_out_dirs_from_check: bool`"""

    @load_out_dirs_from_check.setter
    def load_out_dirs_from_check(self, value: bool) -> None: ...

    def __new__(cls, load_out_dirs_from_check: bool, with_proc_macro_server: ProcMacroServerChoice, prefill_caches: bool, num_worker_threads: int, proc_macro_processes: int) -> Self:
        """The struct literal. Rust gives the struct no default, so every field must be given."""

    @property
    def num_worker_threads(self) -> int:
        """`pub num_worker_threads: usize`"""

    @num_worker_threads.setter
    def num_worker_threads(self, value: int) -> None: ...

    @property
    def prefill_caches(self) -> bool:
        """`pub prefill_caches: bool`"""

    @prefill_caches.setter
    def prefill_caches(self, value: bool) -> None: ...

    @property
    def proc_macro_processes(self) -> int:
        """`pub proc_macro_processes: usize`"""

    @proc_macro_processes.setter
    def proc_macro_processes(self, value: int) -> None: ...

    @property
    def with_proc_macro_server(self) -> ProcMacroServerChoice:
        """`pub with_proc_macro_server: ProcMacroServerChoice`"""

    @with_proc_macro_server.setter
    def with_proc_macro_server(self, value: ProcMacroServerChoice) -> None: ...

@final
class MacroCallId:
    """`hir::MacroCallId`"""

@final
class ManifestPath:
    """`project_model::ManifestPath`"""

    def parent(self) -> AbsPath:
        """`pub fn parent(&self) -> &AbsPath`, lent by this `ManifestPath`."""

@final
class Module:
    """`hir::Module`"""

    def children(self, db: RootDatabase) -> list[Module]:
        """`pub fn children(self, db: &dyn HirDatabase) -> impl Iterator<Item = Module>`, as a list."""

    def declaration_source(self, db: RootDatabase) -> InFileAstModule | None:
        """`pub fn declaration_source(self, db: &dyn HirDatabase) -> Option<InFile<ast::Module>>`"""

    def declarations(self, db: RootDatabase) -> list[ModuleDef]:
        """`pub fn declarations(self, db: &dyn HirDatabase) -> Vec<ModuleDef>`"""

    def definition_source(self, db: RootDatabase) -> InFileModuleSource:
        """`pub fn definition_source(self, db: &dyn HirDatabase) -> InFile<ModuleSource>`"""

    def __eq__(self, other: object, /) -> bool:
        """`impl PartialEq for Module`"""

    def impl_defs(self, db: RootDatabase) -> list[Impl]:
        """`pub fn impl_defs(self, db: &dyn HirDatabase) -> Vec<Impl>`"""

    def is_inline(self, db: RootDatabase) -> bool:
        """`pub fn is_inline(self, db: &dyn HirDatabase) -> bool`"""

    def krate(self, db: RootDatabase) -> Crate:
        """`pub fn krate(self, db: &dyn HirDatabase) -> Crate`"""

    def name(self, db: RootDatabase) -> Name | None:
        """`pub fn name(self, db: &dyn HirDatabase) -> Option<Name>`"""

    def parent(self, db: RootDatabase) -> Module | None:
        """`pub fn parent(self, db: &dyn HirDatabase) -> Option<Module>`"""

@final
class ModuleDef:
    """`hir::ModuleDef`"""

    def as_Adt(self) -> Adt | None:
        """The variant `ModuleDef::Adt(Adt)`, read: its `Adt` if the value is that variant, otherwise `None`."""

    @staticmethod
    def Adt(value: Adt) -> ModuleDef:
        """The variant `ModuleDef::Adt(Adt)`, made."""

    def canonical_path(self, db: RootDatabase, edition: Edition) -> str | None:
        """`pub fn canonical_path(&self, db: &dyn HirDatabase, edition: Edition) -> Option<String>`"""

    def as_EnumVariant(self) -> EnumVariant | None:
        """The variant `ModuleDef::EnumVariant(EnumVariant)`, read: its `EnumVariant` if the value is that variant, otherwise `None`."""

    def as_Function(self) -> Function | None:
        """The variant `ModuleDef::Function(Function)`, read: its `Function` if the value is that variant, otherwise `None`."""

    def module(self, db: RootDatabase) -> Module | None:
        """`pub fn module(self, db: &dyn HirDatabase) -> Option<Module>`"""

    def as_Trait(self) -> Trait | None:
        """The variant `ModuleDef::Trait(Trait)`, read: its `Trait` if the value is that variant, otherwise `None`."""

    @staticmethod
    def Trait(value: Trait) -> ModuleDef:
        """The variant `ModuleDef::Trait(Trait)`, made."""

    def as_Module(self) -> Module | None:
        """The `ModuleDef::Module` variant, read."""

    def as_Const(self) -> Const | None:
        """The `ModuleDef::Const` variant, read."""

    def as_Static(self) -> Static | None:
        """The `ModuleDef::Static` variant, read."""

    def as_TypeAlias(self) -> TypeAlias | None:
        """The `ModuleDef::TypeAlias` variant, read."""

    def as_Macro(self) -> Macro | None:
        """The `ModuleDef::Macro` variant, read."""

    @staticmethod
    def Function(value: Function) -> ModuleDef:
        """The `ModuleDef::Function` variant, made."""


@final
class Mutability:
    """`hir::Mutability`"""

    def is_mut(self) -> bool:
        """`pub fn is_mut(&self) -> bool`"""

@final
class Name:
    """`hir::Name`"""

    def as_str(self) -> str:
        """`pub fn as_str(&self) -> &str`"""

@final
class Package:
    """`project_model::Package`"""

@final
class PackageData:
    """`project_model::PackageData`"""

    @property
    def dependencies(self) -> list[PackageDependency]:
        """`pub dependencies: Vec<PackageDependency>`, each lent by this `PackageData`."""

    @property
    def is_member(self) -> bool:
        """`pub is_member: bool`"""

    @property
    def manifest(self) -> ManifestPath:
        """`pub manifest: ManifestPath`, lent by this `PackageData`."""

    @property
    def name(self) -> str:
        """`pub name: String`"""

    @property
    def targets(self) -> list[Target]:
        """`pub targets: Vec<Target>`"""

@final
class PackageDependency:
    """`project_model::PackageDependency`"""

    @property
    def kind(self) -> DepKind:
        """`pub kind: DepKind`, lent by this `PackageDependency`."""

    @property
    def name(self) -> str:
        """`pub name: String`"""

    @property
    def pkg(self) -> Package:
        """`pub pkg: Package`"""

@final
class Param:
    """`hir::Param<'db>`. It borrows the database, so it holds the database's Python object."""

    def ty(self) -> Type:
        """`pub fn ty(&self) -> &Type<'db>`, lent by this `Param`."""

@final
class PathResolution:
    """`hir::PathResolution<'db>`. It borrows the database, so it holds the database's Python object."""

    def as_Def(self) -> ModuleDef | None:
        """The variant `PathResolution::Def(ModuleDef)`, read: its `ModuleDef` if the value is that variant, otherwise `None`."""

    def as_SelfType(self) -> Impl | None:
        """The variant `PathResolution::SelfType(Impl)`, read: its `Impl` if the value is that variant, otherwise `None`."""

    def as_TypeParam(self) -> TypeParam | None:
        """The `PathResolution::TypeParam` variant, read."""

    def as_ConstParam(self) -> ConstParam | None:
        """The `PathResolution::ConstParam` variant, read."""

    def as_Local(self) -> Local | None:
        """The `PathResolution::Local` variant, read."""


@final
class ProcMacroClient:
    """`proc_macro_api::ProcMacroClient`"""

@final
class ProcMacroServerChoice:
    """`load_cargo::ProcMacroServerChoice`"""

    @staticmethod
    def None_() -> ProcMacroServerChoice:
        """The variant `ProcMacroServerChoice::None`, made from Python. Python cannot name a method `None`, so it is `None_`."""

    @staticmethod
    def Sysroot() -> ProcMacroServerChoice:
        """The variant `ProcMacroServerChoice::Sysroot`, made from Python."""

@final
class ProjectManifest:
    """`project_model::ProjectManifest`"""

    @staticmethod
    def discover_single(path: AbsPath) -> ProjectManifest:
        """`pub fn discover_single(path: &AbsPath) -> anyhow::Result<ProjectManifest>`"""

    @staticmethod
    def from_manifest_file(path: AbsPathBuf) -> ProjectManifest:
        """`pub fn from_manifest_file(path: AbsPathBuf) -> anyhow::Result<ProjectManifest>`. It takes the path, so it is given a copy."""

@final
class ProjectWorkspace:
    """`project_model::ProjectWorkspace`"""

    @property
    def kind(self) -> ProjectWorkspaceKind:
        """`pub kind: ProjectWorkspaceKind`. Copied, because `set_build_scripts` changes the `ProjectWorkspace`, so it cannot lend its parts."""

    @staticmethod
    def load(manifest: ProjectManifest, config: CargoConfig, progress: _Callable[[str], object]) -> ProjectWorkspace:
        """`pub fn load(manifest: ProjectManifest, config: &CargoConfig, progress: &(dyn Fn(String) + Sync)) -> anyhow::Result<ProjectWorkspace>`  `progress` is a Python callable that takes a string. If it raises, the binding panics."""

    def run_build_scripts(self, config: CargoConfig, progress: _Callable[[str], object]) -> WorkspaceBuildScripts:
        """`pub fn run_build_scripts(&self, config: &CargoConfig, progress: &dyn Fn(String)) -> anyhow::Result<WorkspaceBuildScripts>`  `progress` is a Python callable that takes a string. If it raises, the binding panics."""

    def set_build_scripts(self, bs: WorkspaceBuildScripts) -> None:
        """`pub fn set_build_scripts(&mut self, bs: WorkspaceBuildScripts)`. It takes the value, so it is given a copy."""

    @property
    def sysroot(self) -> Sysroot:
        """`pub sysroot: Sysroot`. Copied, because `set_build_scripts` changes the `ProjectWorkspace`, so it cannot lend its parts."""

@final
class ProjectWorkspaceKind:
    """`project_model::ProjectWorkspaceKind`"""

    def as_Cargo(self) -> ProjectWorkspaceKindCargo | None:
        """The variant `ProjectWorkspaceKind::Cargo { .. }`, read: its fields, lent by this `ProjectWorkspaceKind`, if the value is that variant, otherwise `None`."""

@final
class ProjectWorkspaceKindCargo:
    """The variant `ProjectWorkspaceKind::Cargo`, lent by the `ProjectWorkspaceKind` that is that variant."""

    @property
    def cargo(self) -> CargoWorkspace:
        """The field `cargo: CargoWorkspace` of `ProjectWorkspaceKind::Cargo`, lent by this variant."""

    @property
    def error(self) -> str | None:
        """The field `error: Option<Arc<anyhow::Error>>` of `ProjectWorkspaceKind::Cargo`, as the text of the error with its causes."""

@final
class RootDatabase:
    """The database every call of `hir` reads from. It is never changed after it is made, so the values that borrow from it stay valid while their Python objects hold it. `ide_db::RootDatabase`"""

@final
class RustLibSource:
    """`project_model::RustLibSource`"""

    def as_Discover(self) -> bool:
        """The variant `RustLibSource::Discover`, read: whether the value is that variant, which holds nothing."""

    @staticmethod
    def Discover() -> RustLibSource:
        """The variant `RustLibSource::Discover`, made from Python."""

    @staticmethod
    def Path(path: AbsPathBuf) -> RustLibSource:
        """The variant `RustLibSource::Path(AbsPathBuf)`, made. It takes the path, so it is given a copy."""

@final
class SemanticsImpl:
    """`hir::SemanticsImpl<'db>`, lent by a `Semantics`."""

    @overload
    def descend_node_into_attributes(self, node: AstMacroCall) -> list[AstMacroCall]:
        """`SemanticsImpl::descend_node_into_attributes` for `ast::MacroCall`."""

    @overload
    def descend_node_into_attributes(self, node: AstExpr) -> list[AstExpr]:
        """`SemanticsImpl::descend_node_into_attributes` for `ast::Expr`."""

    @overload
    def descend_node_into_attributes(self, node: AstPat) -> list[AstPat]:
        """`SemanticsImpl::descend_node_into_attributes` for `ast::Pat`."""

    @overload
    def descend_node_into_attributes(self, node: AstPath) -> list[AstPath]:
        """`SemanticsImpl::descend_node_into_attributes` for `ast::Path`."""

    def expand_macro_call(self, macro_call: AstMacroCall) -> InFileSyntaxNode | None:
        """`pub fn expand_macro_call(&self, macro_call: &ast::MacroCall) -> Option<InFile<SyntaxNode>>`"""

    @overload
    def original_ast_node(self, node: AstFn, /) -> AstFn | None:
        """`pub fn original_ast_node<N: AstNode>(&self, node: N) -> Option<N>`, with `N` an `ast::Fn`, an `ast::Adt`, an `ast::Impl`, an `ast::Trait`, an `ast::Module`, an `ast::RecordField`, an `ast::TupleField` or an `ast::Variant`. It takes the node, so it is given a copy."""

    @overload
    def original_ast_node(self, node: AstAdt, /) -> AstAdt | None:
        """`pub fn original_ast_node<N: AstNode>(&self, node: N) -> Option<N>`, with `N` an `ast::Fn`, an `ast::Adt`, an `ast::Impl`, an `ast::Trait`, an `ast::Module`, an `ast::RecordField`, an `ast::TupleField` or an `ast::Variant`. It takes the node, so it is given a copy."""

    @overload
    def original_ast_node(self, node: AstImpl, /) -> AstImpl | None:
        """`pub fn original_ast_node<N: AstNode>(&self, node: N) -> Option<N>`, with `N` an `ast::Fn`, an `ast::Adt`, an `ast::Impl`, an `ast::Trait`, an `ast::Module`, an `ast::RecordField`, an `ast::TupleField` or an `ast::Variant`. It takes the node, so it is given a copy."""

    @overload
    def original_ast_node(self, node: AstTrait, /) -> AstTrait | None:
        """`pub fn original_ast_node<N: AstNode>(&self, node: N) -> Option<N>`, with `N` an `ast::Fn`, an `ast::Adt`, an `ast::Impl`, an `ast::Trait`, an `ast::Module`, an `ast::RecordField`, an `ast::TupleField` or an `ast::Variant`. It takes the node, so it is given a copy."""

    @overload
    def original_ast_node(self, node: AstModule, /) -> AstModule | None:
        """`pub fn original_ast_node<N: AstNode>(&self, node: N) -> Option<N>`, with `N` an `ast::Fn`, an `ast::Adt`, an `ast::Impl`, an `ast::Trait`, an `ast::Module`, an `ast::RecordField`, an `ast::TupleField` or an `ast::Variant`. It takes the node, so it is given a copy."""

    @overload
    def original_ast_node(self, node: AstRecordField, /) -> AstRecordField | None:
        """`pub fn original_ast_node<N: AstNode>(&self, node: N) -> Option<N>`, with `N` an `ast::Fn`, an `ast::Adt`, an `ast::Impl`, an `ast::Trait`, an `ast::Module`, an `ast::RecordField`, an `ast::TupleField` or an `ast::Variant`. It takes the node, so it is given a copy."""

    @overload
    def original_ast_node(self, node: AstTupleField, /) -> AstTupleField | None:
        """`pub fn original_ast_node<N: AstNode>(&self, node: N) -> Option<N>`, with `N` an `ast::Fn`, an `ast::Adt`, an `ast::Impl`, an `ast::Trait`, an `ast::Module`, an `ast::RecordField`, an `ast::TupleField` or an `ast::Variant`. It takes the node, so it is given a copy."""

    @overload
    def original_ast_node(self, node: AstVariant, /) -> AstVariant | None:
        """`pub fn original_ast_node<N: AstNode>(&self, node: N) -> Option<N>`, with `N` an `ast::Fn`, an `ast::Adt`, an `ast::Impl`, an `ast::Trait`, an `ast::Module`, an `ast::RecordField`, an `ast::TupleField` or an `ast::Variant`. It takes the node, so it is given a copy."""

    def original_range_opt(self, node: SyntaxNode) -> FileRange | None:
        """`pub fn original_range_opt(&self, node: &SyntaxNode) -> Option<FileRange>`"""

    def parse_guess_edition(self, file_id: FileId) -> AstSourceFile:
        """`pub fn parse_guess_edition(&self, file_id: FileId) -> ast::SourceFile`"""

    def record_pattern_missing_fields(self, pattern: AstRecordPat) -> list[tuple[Field, Type]]:
        """`pub fn record_pattern_missing_fields(&self, pattern: &ast::RecordPat) -> Vec<(Field, Type<'db>)>`"""

    def resolve_method_call(self, call: AstMethodCallExpr) -> Function | None:
        """`pub fn resolve_method_call(&self, call: &ast::MethodCallExpr) -> Option<Function>`"""

    def resolve_path(self, path: AstPath) -> PathResolution | None:
        """`pub fn resolve_path(&self, path: &ast::Path) -> Option<PathResolution<'db>>`"""

    @overload
    def source(self, def_: Const, /) -> InFileAstConst | None:
        """`SemanticsImpl::source` for `Const`."""

    @overload
    def source(self, def_: Static, /) -> InFileAstStatic | None:
        """`SemanticsImpl::source` for `Static`."""

    @overload
    def source(self, def_: TypeAlias, /) -> InFileAstTypeAlias | None:
        """`SemanticsImpl::source` for `TypeAlias`."""

    @overload
    def source(self, def_: Macro, /) -> InFileEitherAstMacroAstFn | None:
        """`SemanticsImpl::source` for `Macro`."""

    @overload
    def source(self, def_: TypeOrConstParam, /) -> InFileEitherAstTypeOrConstParamAstTrait | None:
        """`SemanticsImpl::source` for `TypeOrConstParam`."""

    @overload
    def source(self, def_: Function, /) -> InFileAstFn | None:
        """`SemanticsImpl::source` for `Function`."""

    @overload
    def source(self, def_: Adt, /) -> InFileAstAdt | None:
        """`SemanticsImpl::source` for `Adt`."""

    @overload
    def source(self, def_: Impl, /) -> InFileAstImpl | None:
        """`SemanticsImpl::source` for `Impl`."""

    @overload
    def source(self, def_: Trait, /) -> InFileAstTrait | None:
        """`SemanticsImpl::source` for `Trait`."""

    @overload
    def source(self, def_: Field, /) -> InFileFieldSource | None:
        """`SemanticsImpl::source` for `Field`."""

    @overload
    def source(self, def_: EnumVariant, /) -> InFileAstVariant | None:
        """`SemanticsImpl::source` for `EnumVariant`."""

    @overload
    def to_def(self, src: AstTypeAlias) -> TypeAlias | None:
        """`SemanticsImpl::to_def` for `ast::TypeAlias`."""

    @overload
    def to_def(self, src: AstConst) -> Const | None:
        """`SemanticsImpl::to_def` for `ast::Const`."""

    @overload
    def to_def(self, src: AstStatic) -> Static | None:
        """`SemanticsImpl::to_def` for `ast::Static`."""

    @overload
    def to_def(self, src: AstFn) -> Function | None:
        """`SemanticsImpl::to_def` for `ast::Fn`."""

    @overload
    def to_def(self, src: AstAdt) -> Adt | None:
        """`SemanticsImpl::to_def` for `ast::Adt`."""

    @overload
    def to_def(self, src: AstImpl) -> Impl | None:
        """`SemanticsImpl::to_def` for `ast::Impl`."""

    @overload
    def to_def(self, src: AstTrait) -> Trait | None:
        """`SemanticsImpl::to_def` for `ast::Trait`."""

    def type_of_pat(self, pat: AstPat) -> TypeInfo | None:
        """`pub fn type_of_pat(&self, pat: &ast::Pat) -> Option<TypeInfo<'db>>`"""

    def resolve_field(self, field: AstFieldExpr) -> EitherFieldTupleField | None:
        """`pub fn resolve_field(&self, field: &ast::FieldExpr) -> Option<Either<Field, TupleField<'db>>>`"""

    def type_of_expr(self, expr: AstExpr) -> TypeInfo | None:
        """`fn type_of_expr(&self, expr: &ast::Expr) -> Option<TypeInfo<'db>>`"""


    def resolve_bind_pat_to_const(self, pat: AstIdentPat) -> ModuleDef | None:
        """`fn resolve_bind_pat_to_const(&self, pat: &ast::IdentPat) -> Option<ModuleDef>`"""

    def resolve_record_field(self, field: AstRecordExprField) -> tuple[Field, Local | None, Type] | None:
        """`fn resolve_record_field(&self, field: &ast::RecordExprField) -> Option<(Field, Option<Local<'db>>, Type<'db>)>`"""

    def impl_generated_from_derive(self, impl_: Impl) -> Adt | None:
        """`fn impl_generated_from_derive(&self, impl_: Impl) -> Option<Adt>`"""


@final
class SemanticsRootDatabase:
    """`hir::Semantics<'db, RootDatabase>`. It borrows the database, so it holds the database's Python object."""

    def deref(self) -> SemanticsImpl:
        """`impl Deref<Target = SemanticsImpl<'db>> for Semantics<'db, DB>`, lent by this `Semantics`."""

    def file_to_module_defs(self, file: FileId) -> Iterator[Module]:
        """`pub fn file_to_module_defs(&self, file: impl Into<FileId>) -> impl Iterator<Item = Module>`, with `file` a `FileId`, lent by this `Semantics`."""

    def lint_attrs(self, file_id: FileId, krate: Crate, item: AstAnyHasAttrs) -> Iterator[tuple[LintAttr, str]]:
        """`pub fn lint_attrs(&self, file_id: FileId, krate: Crate, item: ast::AnyHasAttrs) -> impl DoubleEndedIterator<Item = (LintAttr, SmolStr)>`, lent by this `Semantics`."""

    @staticmethod
    def new(db: RootDatabase) -> SemanticsRootDatabase:
        """`pub fn new(db: &DB) -> Semantics<'_, DB>`, with `DB` the `RootDatabase`."""

    def hir_file_for(self, syntax_node: SyntaxNode) -> HirFileId:
        """`fn hir_file_for(&self, syntax_node: &SyntaxNode) -> HirFileId`"""


@final
class Struct:
    """`hir::Struct`"""

    def fields(self, db: RootDatabase) -> list[Field]:
        """`pub fn fields(self, db: &dyn HirDatabase) -> Vec<Field>`"""

    def kind(self, db: RootDatabase) -> StructKind:
        """`pub fn kind(self, db: &dyn HirDatabase) -> StructKind`"""

@final
class StructKind:
    """`hir::StructKind`"""

    def as_Record(self) -> bool:
        """The variant `StructKind::Record`, read: whether the value is that variant, which holds nothing."""

    def as_Tuple(self) -> bool:
        """The variant `StructKind::Tuple`, read: whether the value is that variant, which holds nothing."""

    def as_Unit(self) -> bool:
        """The variant `StructKind::Unit`, read: whether the value is that variant, which holds nothing."""

@final
class Symbol:
    """`intern::Symbol`"""

    @staticmethod
    def intern(s: str) -> Symbol:
        """`pub fn intern(s: &str) -> Self`"""

@final
class SyntaxElement:
    """`syntax::SyntaxElement`, which is `NodeOrToken<SyntaxNode, SyntaxToken>`"""

    def into_node(self) -> SyntaxNode | None:
        """`pub fn into_node(self) -> Option<N>`, with `N` the `SyntaxNode`. It takes the value, so it is given a copy."""

    def into_token(self) -> SyntaxToken | None:
        """`pub fn into_token(self) -> Option<SyntaxToken>`. It takes the value, so it is given a copy."""

    def as_Token(self) -> SyntaxToken | None:
        """The variant `NodeOrToken::Token(T)`, read: its `SyntaxToken`, lent by this `SyntaxElement`, if the value is that variant, otherwise `None`."""

@final
class SyntaxError:
    """`syntax::SyntaxError`"""

    def __str__(self) -> str:
        """`impl Display for SyntaxError`"""

    def range(self) -> TextRange:
        """`pub fn range(&self) -> TextRange`"""

@final
class SyntaxKind:
    """`parser::SyntaxKind`"""

    @staticmethod
    def COMMA() -> SyntaxKind:
        """The variant `SyntaxKind::COMMA`, made from Python."""

    def __repr__(self) -> str:
        """`impl Debug for SyntaxKind`"""

    def __eq__(self, other: object, /) -> bool:
        """`impl PartialEq for SyntaxKind`"""

    def __hash__(self) -> int:
        """`impl Hash for SyntaxKind`"""

    def is_trivia(self) -> bool:
        """`pub fn is_trivia(self) -> bool`"""

    @staticmethod
    def WHITESPACE() -> SyntaxKind:
        """The variant `SyntaxKind::WHITESPACE`, made from Python."""

@final
class SyntaxNode:
    """`syntax::SyntaxNode`"""

    def ancestors(self) -> Iterator[SyntaxNode]:
        """`pub fn ancestors(&self) -> impl Iterator<Item = SyntaxNode>`"""

    def children(self) -> Iterator[SyntaxNode]:
        """`pub fn children(&self) -> SyntaxNodeChildren`"""

    def children_with_tokens(self) -> Iterator[SyntaxElement]:
        """`pub fn children_with_tokens(&self) -> SyntaxElementChildren`"""

    def clone(self) -> SyntaxNode:
        """`impl Clone for SyntaxNode`"""

    def descendants(self) -> Iterator[SyntaxNode]:
        """`pub fn descendants(&self) -> impl Iterator<Item = SyntaxNode>`"""

    def descendants_with_tokens(self) -> Iterator[SyntaxElement]:
        """`pub fn descendants_with_tokens(&self) -> impl Iterator<Item = SyntaxElement>`"""

    def __eq__(self, other: object, /) -> bool:
        """`impl PartialEq for SyntaxNode`"""

    def first_token(self) -> SyntaxToken | None:
        """`pub fn first_token(&self) -> Option<SyntaxToken>`"""

    def __hash__(self) -> int:
        """`impl Hash for SyntaxNode`"""

    def kind(self) -> SyntaxKind:
        """`pub fn kind(&self) -> SyntaxKind`"""

    def parent(self) -> SyntaxNode | None:
        """`pub fn parent(&self) -> Option<SyntaxNode>`"""

    def prev_sibling_or_token(self) -> SyntaxElement | None:
        """`pub fn prev_sibling_or_token(&self) -> Option<SyntaxElement>`"""

    def text(self) -> SyntaxText:
        """`pub fn text(&self) -> SyntaxText`"""

    def text_range(self) -> TextRange:
        """`pub fn text_range(&self) -> TextRange`"""

@final
class SyntaxText:
    """`rowan::SyntaxText`"""

    def __str__(self) -> str:
        """`impl Display for SyntaxText`"""

@final
class SyntaxToken:
    """`rowan::SyntaxToken`"""

    def kind(self) -> SyntaxKind:
        """`pub fn kind(&self) -> SyntaxKind`"""

    def prev_sibling_or_token(self) -> SyntaxElement | None:
        """`pub fn prev_sibling_or_token(&self) -> Option<SyntaxElement>`"""

    def text(self) -> str:
        """`pub fn text(&self) -> &str`"""

    def text_range(self) -> TextRange:
        """`pub fn text_range(&self) -> TextRange`"""

@final
class Sysroot:
    """`project_model::Sysroot`"""

    def error(self) -> str | None:
        """`pub fn error(&self) -> Option<&str>`"""

    def is_rust_lib_src_empty(self) -> bool:
        """`pub fn is_rust_lib_src_empty(&self) -> bool`"""

@final
class Target:
    """`project_model::Target`, which is `Idx<TargetData>`"""

@final
class TargetData:
    """`project_model::TargetData`"""

    @property
    def kind(self) -> TargetKind:
        """`pub kind: TargetKind`"""

    @property
    def name(self) -> str:
        """`pub name: String`"""

    @property
    def package(self) -> Package:
        """`pub package: Package`"""

    @property
    def root(self) -> AbsPath:
        """`pub root: AbsPathBuf`, as the `AbsPath` it derefs to, lent by this `TargetData`."""

@final
class TargetDirectoryConfig:
    """`project_model::TargetDirectoryConfig`"""

    @staticmethod
    def Directory(path: str) -> TargetDirectoryConfig:
        """The variant `TargetDirectoryConfig::Directory(Utf8PathBuf)`, made from the path as a string."""

@final
class TargetKind:
    """`project_model::TargetKind`"""

    def as_Bench(self) -> bool:
        """The variant `TargetKind::Bench`, read: whether the value is that variant."""

    def as_Bin(self) -> bool:
        """The variant `TargetKind::Bin`, read: whether the value is that variant."""

    def as_BuildScript(self) -> bool:
        """The variant `TargetKind::BuildScript`, read: whether the value is that variant."""

    def as_Example(self) -> bool:
        """The variant `TargetKind::Example`, read: whether the value is that variant."""

    def as_Lib(self) -> bool | None:
        """The variant `TargetKind::Lib { is_proc_macro: bool }`, read: its `is_proc_macro` if the value is that variant, otherwise `None`."""

    def as_Test(self) -> bool:
        """The variant `TargetKind::Test`, read: whether the value is that variant."""

@final
class TextRange:
    """`text_size::TextRange`"""

    def __repr__(self) -> str:
        """`impl Debug for TextRange`"""

    @staticmethod
    def empty(offset: TextSize) -> TextRange:
        """`pub const fn empty(offset: TextSize) -> TextRange`"""

    def end(self) -> TextSize:
        """`pub const fn end(self) -> TextSize`"""

    def start(self) -> TextSize:
        """`pub const fn start(self) -> TextSize`"""

@final
class TextSize:
    """`text_size::TextSize`"""

    def __repr__(self) -> str:
        """`impl Debug for TextSize`"""

    @staticmethod
    def from_u32(raw: int) -> TextSize:
        """`impl From<u32> for TextSize`: `fn from(raw: u32) -> Self`"""

    def __int__(self) -> int:
        """`impl From<TextSize> for u32`: `fn from(value: TextSize) -> Self`"""

    def sub_TextSize(self, rhs: TextSize) -> TextSize:
        """`impl Sub<TextSize> for TextSize`: `fn sub(self, rhs: TextSize) -> TextSize`"""

@final
class Trait:
    """`hir::Trait`"""

    def visibility(self, db: RootDatabase) -> Visibility:
        """`HasVisibility::visibility(&self, db: &dyn HirDatabase) -> Visibility`"""

    def hir_docs(self, db: RootDatabase) -> Docs | None:
        """`HasAttrs::hir_docs(self, db: &dyn HirDatabase) -> Option<&Docs>`, as a copy."""

    def __eq__(self, other: object, /) -> bool:
        """`impl PartialEq for Trait`"""

    def __hash__(self) -> int:
        """`impl Hash for Trait`"""

    def items(self, db: RootDatabase) -> list[AssocItem]:
        """`pub fn items(self, db: &dyn HirDatabase) -> Vec<AssocItem>`"""

    def module(self, db: RootDatabase) -> Module:
        """`pub fn module(self, db: &dyn HirDatabase) -> Module`"""

    def name(self, db: RootDatabase) -> Name:
        """`pub fn name(self, db: &dyn HirDatabase) -> Name`"""

@final
class TraitRef:
    """`hir::TraitRef<'db>`. It borrows the database, so it holds the database's Python object."""

    def get_type_argument(self, idx: int) -> Type | None:
        """`pub fn get_type_argument(&self, idx: usize) -> Option<Type<'db>>`"""

    def display(self, db: RootDatabase, display_target: DisplayTarget) -> HirDisplayWrapperTraitRef:
        """`impl HirDisplay for TraitRef`: `fn display<'a>(&'a self, db: &'db dyn HirDatabase, display_target: DisplayTarget) -> HirDisplayWrapper<'a, 'db, Self>`, lent by this `TraitRef` and the database."""

@final
class Type:
    """`hir::Type<'db>`. It borrows from the database, so it holds the database's Python object, which keeps the database alive as long as the type."""

    def display(self, db: RootDatabase, display_target: DisplayTarget) -> HirDisplayWrapperType:
        """`impl HirDisplay for Type`: `fn display<'a>(&'a self, db: &'db dyn HirDatabase, display_target: DisplayTarget) -> HirDisplayWrapper<'a, 'db, Self>`, lent by this `Type` and the database."""

    def display_truncated(self, db: RootDatabase, max_size: int | None, display_target: DisplayTarget) -> HirDisplayWrapperType:
        """`impl HirDisplay for Type`: `fn display_truncated<'a>(&'a self, db: &'db dyn HirDatabase, max_size: Option<usize>, display_target: DisplayTarget) -> HirDisplayWrapper<'a, 'db, Self>`, lent by this `Type` and the database."""

    def as_adt(self) -> Adt | None:
        """`pub fn as_adt(&self) -> Option<Adt>`"""

    def as_array(self, db: RootDatabase) -> tuple[Type, int] | None:
        """`pub fn as_array(&self, db: &'db dyn HirDatabase) -> Option<(Self, usize)>`"""

    def as_builtin(self) -> BuiltinType | None:
        """`pub fn as_builtin(&self) -> Option<BuiltinType>`"""

    def as_callable(self, db: RootDatabase) -> Callable | None:
        """`pub fn as_callable(&self, db: &'db dyn HirDatabase) -> Option<Callable<'db>>`"""

    def as_associated_type_parent_trait(self, db: RootDatabase) -> Trait | None:
        """`pub fn as_associated_type_parent_trait(&self, db: &'db dyn HirDatabase) -> Option<Trait>`"""

    def as_dyn_trait(self) -> Trait | None:
        """`pub fn as_dyn_trait(&self) -> Option<Trait>`"""

    def as_impl_traits(self, db: RootDatabase) -> Iterator[Trait] | None:
        """`pub fn as_impl_traits(&self, db: &'db dyn HirDatabase) -> Option<impl Iterator<Item = Trait>>`, the iterator lent by this `Type`."""

    def as_raw_ptr(self) -> tuple[Type, Mutability] | None:
        """`pub fn as_raw_ptr(&self) -> Option<(Type<'db>, Mutability)>`"""

    def as_reference(self) -> tuple[Type, Mutability] | None:
        """`pub fn as_reference(&self) -> Option<(Type<'db>, Mutability)>`"""

    def as_slice(self) -> Type | None:
        """`pub fn as_slice(&self) -> Option<Type<'db>>`"""

    def as_type_param(self, db: RootDatabase) -> TypeParam | None:
        """`pub fn as_type_param(&self, _db: &'db dyn HirDatabase) -> Option<TypeParam>`"""

    def clone(self) -> Type:
        """`impl Clone for Type<'db>`"""

    def future_output(self, db: RootDatabase) -> Type | None:
        """`pub fn future_output(self, db: &'db dyn HirDatabase) -> Option<Type<'db>>`. It takes the type, so it is given a copy."""

    def impls_trait(self, db: RootDatabase, trait_: Trait, args: list[Type]) -> bool:
        """`pub fn impls_trait(&self, db: &'db dyn HirDatabase, trait_: Trait, args: &[Type<'db>]) -> bool`"""

    def is_tuple(self) -> bool:
        """`pub fn is_tuple(&self) -> bool`"""

    def iterator_item(self, db: RootDatabase) -> Type | None:
        """`pub fn iterator_item(self, db: &'db dyn HirDatabase) -> Option<Type<'db>>`. It takes the type, so it is given a copy."""

    def tuple_fields(self, db: RootDatabase) -> list[Type]:
        """`pub fn tuple_fields(&self, _db: &'db dyn HirDatabase) -> Vec<Self>`"""

    def type_arguments(self) -> Iterator[Type]:
        """`pub fn type_arguments(&self) -> impl Iterator<Item = Type<'db>> + '_`, lent by this `Type`."""

    def walk(self, db: RootDatabase, callback: _Callable[[Type], object]) -> None:
        """`pub fn walk(&self, db: &'db dyn HirDatabase, callback: impl FnMut(Type<'db>))`  `callback` is a Python callable that takes a `Type`. If it raises, the binding panics."""

    def type_and_const_arguments(self, db: RootDatabase, display_target: DisplayTarget) -> Iterator[str]:
        """`pub fn type_and_const_arguments<'a>(&'a self, db: &'a dyn HirDatabase, display_target: DisplayTarget) -> impl Iterator<Item = SmolStr> + 'a`, lent by this `Type` and the database."""

    def is_unknown(self) -> bool:
        """`fn is_unknown(&self) -> bool`"""


@final
class TypeInfo:
    """`hir::TypeInfo<'db>`. It borrows the database, so it holds the database's Python object."""

    @property
    def original(self) -> Type:
        """`pub original: Type<'db>`, lent by this `TypeInfo`."""

@final
class TypeOrConstParam:
    """`hir::TypeOrConstParam`"""

    def as_type_param(self, db: RootDatabase) -> TypeParam | None:
        """`pub fn as_type_param(self, db: &dyn HirDatabase) -> Option<TypeParam>`"""

@final
class TypeParam:
    """`hir::TypeParam`"""

    def name(self, db: RootDatabase) -> Name:
        """`pub fn name(self, db: &dyn HirDatabase) -> Name`"""

    def trait_bounds(self, db: RootDatabase) -> list[Trait]:
        """`pub fn trait_bounds(self, db: &dyn HirDatabase) -> Vec<Trait>`"""

    def merge(self) -> TypeOrConstParam:
        """`fn merge(self) -> TypeOrConstParam`"""


@final
class Union:
    """`hir::Union`"""

    def fields(self, db: RootDatabase) -> list[Field]:
        """`pub fn fields(self, db: &dyn HirDatabase) -> Vec<Field>`"""

@final
class Vfs:
    """`vfs::Vfs`"""

    def file_path(self, file_id: FileId) -> VfsPath:
        """`pub fn file_path(&self, file_id: FileId) -> &VfsPath`, lent by this `Vfs`."""

    def iter(self) -> Iterator[tuple[FileId, VfsPath]]:
        """`pub fn iter(&self) -> impl Iterator<Item = (FileId, &VfsPath)> + '_`, lent by this `Vfs`."""

@final
class VfsPath:
    """`vfs::VfsPath`"""

    def as_path(self) -> AbsPath | None:
        """`pub fn as_path(&self) -> Option<&AbsPath>`, lent by this `VfsPath`."""

@final
class Visibility:
    """`hir::Visibility`"""

    def __eq__(self, other: object, /) -> bool:
        """`impl PartialEq for Visibility`"""

    def __hash__(self) -> int:
        """`impl Hash for Visibility`"""

    def as_Module(self) -> Module | None:
        """The variant `Visibility::Module(ModuleId, VisibilityExplicitness)`, read: its module, as the `hir::Module` that `From<ModuleId>` makes of it, if the value is that variant, otherwise `None`."""

    @staticmethod
    def Public() -> Visibility:
        """The variant `Visibility::Public`, made."""

@final
class WideEncoding:
    """`line_index::WideEncoding`"""

    @staticmethod
    def Utf32() -> WideEncoding:
        """The variant `WideEncoding::Utf32`, made."""

@final
class WideLineCol:
    """`line_index::WideLineCol`"""

    @property
    def col(self) -> int:
        """`pub col: u32`"""

@final
class WorkspaceBuildScripts:
    """`project_model::WorkspaceBuildScripts`"""

    def error(self) -> str | None:
        """`pub fn error(&self) -> Option<&str>`"""

def expand_cfg_attr[T](
    attrs: list[AstAttr], cfg_options: CfgOptions, callback: _Callable[[AstMeta, AstAttr], T | None]
) -> T | None:
    """`pub fn expand_cfg_attr<'a, BreakValue>(attrs: impl Iterator<Item = ast::Attr>, cfg_options: impl FnMut() -> &'a CfgOptions, callback: impl FnMut(ast::Meta, ast::Attr) -> ControlFlow<BreakValue>) -> Option<BreakValue>`  `attrs` is a list, and `cfg_options` the options themselves. `callback` is a Python callable that takes a `Meta` and an `Attr`: what it returns breaks, and is returned, unless it is `None`, which continues. If it raises, the binding panics."""

def load_workspace(ws: ProjectWorkspace, extra_env: dict[str, str | None], load_config: LoadCargoConfig) -> tuple[RootDatabase, Vfs, ProcMacroClient | None]:
    """`pub fn load_workspace(ws: ProjectWorkspace, extra_env: &FxHashMap<String, Option<String>>, load_config: &LoadCargoConfig) -> anyhow::Result<(RootDatabase, Vfs, Option<ProcMacroClient>)>`"""

def parse_tt_as_comma_sep_paths(input: AstTokenTree, edition: Edition) -> list[AstPath] | None:
    """`pub fn parse_tt_as_comma_sep_paths(input: ast::TokenTree, edition: Edition) -> Option<Vec<ast::Path>>`. It takes the token tree, so it is given a copy."""

class PanicException(BaseException):
    """What a binding raises when rust-analyzer panics. It is no `Exception`, so `except Exception` lets it pass."""


@final
class Variant:
    """`hir::Variant`"""

    def as_Struct(self) -> Struct | None:
        """The variant `Variant::Struct(Struct)`, read: its `Struct` if the value is that variant, otherwise `None`."""

    def as_Union(self) -> Union | None:
        """The variant `Variant::Union(Union)`, read: its `Union` if the value is that variant, otherwise `None`."""

    def as_EnumVariant(self) -> EnumVariant | None:
        """The variant `Variant::EnumVariant(EnumVariant)`, read: its `EnumVariant` if the value is that variant, otherwise `None`."""


@final
class TypeAlias:
    """`hir::TypeAlias`"""

    def visibility(self, db: RootDatabase) -> Visibility:
        """`HasVisibility::visibility(&self, db: &dyn HirDatabase) -> Visibility`"""

    def hir_docs(self, db: RootDatabase) -> Docs | None:
        """`HasAttrs::hir_docs(self, db: &dyn HirDatabase) -> Option<&Docs>`, as a copy."""

    def name(self, db: RootDatabase) -> Name:
        """`pub fn name(self, db: &dyn HirDatabase) -> Name`"""

    def ty(self, db: RootDatabase) -> Type:
        """`pub fn ty(self, db: &dyn HirDatabase) -> Type<'_>`"""


@final
class EitherFieldTupleField:
    """`Either<hir::Field, hir::TupleField<'db>>`. It borrows from the database, so it holds the database's Python object."""

    def as_Left(self) -> Field | None:
        """The variant `Either::Left(Field)`, read: its `Field` if the value is that variant, otherwise `None`."""

@final
class AstParamList:
    """`syntax::ast::ParamList`"""

    def params(self) -> Iterator[AstParam]:
        """`fn params(&self) -> AstChildren<Param>`"""


@final
class AstParam:
    """`syntax::ast::Param`"""

    def pat(self) -> AstPat | None:
        """`fn pat(&self) -> Option<Pat>`"""


@final
class AstMatchArm:
    """`syntax::ast::MatchArm`"""

    def pat(self) -> AstPat | None:
        """`fn pat(&self) -> Option<Pat>`"""

    def guard(self) -> AstMatchGuard | None:
        """`fn guard(&self) -> Option<MatchGuard>`"""

    def expr(self) -> AstExpr | None:
        """`fn expr(&self) -> Option<Expr>`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstMatchArm | None:
        """`AstNode::cast(syntax: SyntaxNode) -> Option<MatchArm>`. It takes a copy of the node."""


@final
class AstMatchGuard:
    """`syntax::ast::MatchGuard`"""

    def condition(self) -> AstExpr | None:
        """`fn condition(&self) -> Option<Expr>`"""


@final
class AstLetElse:
    """`syntax::ast::LetElse`"""

    def block_expr(self) -> AstBlockExpr | None:
        """`fn block_expr(&self) -> Option<BlockExpr>`"""


@final
class AstBinExpr:
    """`syntax::ast::BinExpr`"""

    def op_kind(self) -> AstBinaryOp | None:
        """`fn op_kind(&self) -> Option<BinaryOp>`"""

    def lhs(self) -> AstExpr | None:
        """`fn lhs(&self) -> Option<Expr>`"""

    def rhs(self) -> AstExpr | None:
        """`fn rhs(&self) -> Option<Expr>`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstBinExpr | None:
        """`AstNode::cast(syntax: SyntaxNode) -> Option<BinExpr>`. It takes a copy of the node."""


@final
class AstArrayExpr:
    """`syntax::ast::ArrayExpr`"""

    def kind(self) -> AstArrayExprKind:
        """`fn kind(&self) -> ArrayExprKind`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstArrayExpr | None:
        """`AstNode::cast(syntax: SyntaxNode) -> Option<ArrayExpr>`. It takes a copy of the node."""


@final
class AstTupleStructPat:
    """`syntax::ast::TupleStructPat`"""

    def fields(self) -> Iterator[AstPat]:
        """`fn fields(&self) -> AstChildren<Pat>`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstTupleStructPat | None:
        """`AstNode::cast(syntax: SyntaxNode) -> Option<TupleStructPat>`. It takes a copy of the node."""


@final
class AstTuplePat:
    """`syntax::ast::TuplePat`"""

    def fields(self) -> Iterator[AstPat]:
        """`fn fields(&self) -> AstChildren<Pat>`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstTuplePat | None:
        """`AstNode::cast(syntax: SyntaxNode) -> Option<TuplePat>`. It takes a copy of the node."""


@final
class AstSlicePat:
    """`syntax::ast::SlicePat`"""

    def pats(self) -> Iterator[AstPat]:
        """`fn pats(&self) -> AstChildren<Pat>`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstSlicePat | None:
        """`AstNode::cast(syntax: SyntaxNode) -> Option<SlicePat>`. It takes a copy of the node."""


@final
class AstOrPat:
    """`syntax::ast::OrPat`"""

    def pats(self) -> Iterator[AstPat]:
        """`fn pats(&self) -> AstChildren<Pat>`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstOrPat | None:
        """`AstNode::cast(syntax: SyntaxNode) -> Option<OrPat>`. It takes a copy of the node."""


@final
class AstRefPat:
    """`syntax::ast::RefPat`"""

    def pat(self) -> AstPat | None:
        """`fn pat(&self) -> Option<Pat>`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstRefPat | None:
        """`AstNode::cast(syntax: SyntaxNode) -> Option<RefPat>`. It takes a copy of the node."""


@final
class AstParenPat:
    """`syntax::ast::ParenPat`"""

    def pat(self) -> AstPat | None:
        """`fn pat(&self) -> Option<Pat>`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstParenPat | None:
        """`AstNode::cast(syntax: SyntaxNode) -> Option<ParenPat>`. It takes a copy of the node."""


@final
class AstExternBlock:
    """`syntax::ast::ExternBlock`"""

    def extern_item_list(self) -> AstExternItemList | None:
        """`fn extern_item_list(&self) -> Option<ExternItemList>`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstExternBlock | None:
        """`AstNode::cast(syntax: SyntaxNode) -> Option<ExternBlock>`. It takes a copy of the node."""


@final
class AstExternItemList:
    """`syntax::ast::ExternItemList`"""

    def extern_items(self) -> Iterator[AstExternItem]:
        """`fn extern_items(&self) -> AstChildren<ExternItem>`"""


@final
class AstExternItem:
    """`syntax::ast::ExternItem`"""

    def syntax(self) -> SyntaxNode:
        """`fn syntax(&self) -> &SyntaxNode`, lent by the receiver."""


@final
class AstRecordExprField:
    """`syntax::ast::RecordExprField`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstRecordExprField | None:
        """`AstNode::cast(syntax: SyntaxNode) -> Option<RecordExprField>`. It takes a copy of the node."""

    def field_name(self) -> AstNameRef | None:
        """`fn field_name(&self) -> Option<NameRef>`"""


@final
class AstElseBranch:
    """`syntax::ast::ElseBranch`"""

    def syntax(self) -> SyntaxNode:
        """`fn syntax(&self) -> &SyntaxNode`, lent by the receiver."""


@final
class AstArrayExprKind:
    """`syntax::ast::ArrayExprKind`"""

    def as_Repeat(self) -> tuple[AstExpr | None, AstExpr | None] | None:
        """The `ArrayExprKind::Repeat` variant, read."""

    def as_ElementList(self) -> Iterator[AstExpr] | None:
        """The `ArrayExprKind::ElementList` variant, read."""


@final
class AstBlockModifier:
    """`syntax::ast::BlockModifier`"""

    def as_Async(self) -> SyntaxToken | None:
        """The `BlockModifier::Async` variant, read."""

    def as_Unsafe(self) -> SyntaxToken | None:
        """The `BlockModifier::Unsafe` variant, read."""

    def as_Const(self) -> SyntaxToken | None:
        """The `BlockModifier::Const` variant, read."""

    def as_AsyncGen(self) -> SyntaxToken | None:
        """The `BlockModifier::AsyncGen` variant, read."""

    def as_Gen(self) -> SyntaxToken | None:
        """The `BlockModifier::Gen` variant, read."""

    def as_Label(self) -> AstLabel | None:
        """The `BlockModifier::Label` variant, read."""

    def as_Try(self) -> tuple[SyntaxToken, SyntaxToken | None, AstType | None] | None:
        """The `BlockModifier::Try` variant, read."""


@final
class AstBinaryOp:
    """`syntax::ast::BinaryOp`"""

    def __str__(self) -> str:
        """`impl Display for BinaryOp`"""


@final
class AstFieldKind:
    """`syntax::ast::FieldKind`"""

    def as_Index(self) -> SyntaxToken | None:
        """The `FieldKind::Index` variant, read."""

    def as_Name(self) -> AstNameRef | None:
        """The `FieldKind::Name` variant, read."""


@final
class AstNameOrNameRef:
    """`syntax::ast::NameOrNameRef`"""

    def as_name(self) -> Name:
        """`AsName::as_name(&self) -> Name`."""

    def syntax(self) -> SyntaxNode:
        """`fn syntax(&self) -> &SyntaxNode`, lent by the receiver."""


@final
class AstType:
    """`syntax::ast::Type`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstType | None:
        """`AstNode::cast(syntax: SyntaxNode) -> Option<Type>`. It takes a copy of the node."""

    def syntax(self) -> SyntaxNode:
        """`fn syntax(&self) -> &SyntaxNode`, lent by the receiver."""


@final
class AstLabel:
    """`syntax::ast::Label`"""

    def syntax(self) -> SyntaxNode:
        """`fn syntax(&self) -> &SyntaxNode`, lent by the receiver."""


@final
class Const:
    """`hir::Const`"""

    def visibility(self, db: RootDatabase) -> Visibility:
        """`HasVisibility::visibility(&self, db: &dyn HirDatabase) -> Visibility`"""

    def hir_docs(self, db: RootDatabase) -> Docs | None:
        """`HasAttrs::hir_docs(self, db: &dyn HirDatabase) -> Option<&Docs>`, as a copy."""

    def ty(self, db: RootDatabase) -> Type:
        """`fn ty(self, db: &dyn HirDatabase) -> Type<'_>`"""

@final
class Static:
    """`hir::Static`"""

    def visibility(self, db: RootDatabase) -> Visibility:
        """`HasVisibility::visibility(&self, db: &dyn HirDatabase) -> Visibility`"""

    def hir_docs(self, db: RootDatabase) -> Docs | None:
        """`HasAttrs::hir_docs(self, db: &dyn HirDatabase) -> Option<&Docs>`, as a copy."""

    def ty(self, db: RootDatabase) -> Type:
        """`fn ty(self, db: &dyn HirDatabase) -> Type<'_>`"""

@final
class Macro:
    """`hir::Macro`"""

@final
class ConstParam:
    """`hir::ConstParam`"""

    def merge(self) -> TypeOrConstParam:
        """`fn merge(self) -> TypeOrConstParam`"""


@final
class Local:
    """`hir::Local<'db>`, retaining the database that owns the local."""

    def primary_source(self, db: RootDatabase) -> LocalSource:
        """`fn primary_source(self, db: &dyn HirDatabase) -> LocalSource<'db>`"""


@final
class LocalSource:
    """`hir::LocalSource<'db>`, retaining the database that owns the local."""

    def syntax(self) -> SyntaxNode:
        """`fn syntax(&self) -> &SyntaxNode`, lent by the receiver."""

    def name(self) -> InFileAstName | None:
        """`fn name(&self) -> Option<InFile<ast::Name>>`"""

    def file(self) -> HirFileId:
        """`fn file(&self) -> HirFileId`"""


@final
class InFileAstConst:
    """`hir::InFile<syntax::ast::Const>`"""

    def syntax(self) -> InFileSyntaxNodeRef:
        """`fn syntax(&self) -> InFile<&SyntaxNode>`, lent by the receiver."""


@final
class InFileAstStatic:
    """`hir::InFile<syntax::ast::Static>`"""

    def syntax(self) -> InFileSyntaxNodeRef:
        """`fn syntax(&self) -> InFile<&SyntaxNode>`, lent by the receiver."""


@final
class InFileAstTypeAlias:
    """`hir::InFile<syntax::ast::TypeAlias>`"""

    def syntax(self) -> InFileSyntaxNodeRef:
        """`fn syntax(&self) -> InFile<&SyntaxNode>`, lent by the receiver."""


@final
class InFileEitherAstMacroAstFn:
    """`hir::InFile<either::Either<syntax::ast::Macro, syntax::ast::Fn>>`"""

    def syntax(self) -> InFileSyntaxNodeRef:
        """`fn syntax(&self) -> InFile<&SyntaxNode>`, lent by the receiver."""


@final
class InFileEitherAstTypeOrConstParamAstTrait:
    """`hir::InFile<either::Either<syntax::ast::TypeOrConstParam, syntax::ast::Trait>>`"""

    def syntax(self) -> InFileSyntaxNodeRef:
        """`fn syntax(&self) -> InFile<&SyntaxNode>`, lent by the receiver."""


@final
class InFileAstName:
    """`hir::InFile<syntax::ast::Name>`"""

    def syntax(self) -> InFileSyntaxNodeRef:
        """`fn syntax(&self) -> InFile<&SyntaxNode>`, lent by the receiver."""


@final
class InFileSyntaxNodeRef:
    """`hir::InFile<&SyntaxNode>`, retaining the original syntax tree."""

    @property
    def file_id(self) -> HirFileId:
        """`pub file_id: HirFileId`"""

    @property
    def value(self) -> SyntaxNode:
        """`pub value: &SyntaxNode`"""


@final
class AstTypeAlias:
    """`syntax::ast::TypeAlias`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstTypeAlias | None:
        """`AstNode::cast(syntax: SyntaxNode) -> Option<TypeAlias>`. It takes a copy of the node."""


@final
class AstConst:
    """`syntax::ast::Const`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstConst | None:
        """`AstNode::cast(syntax: SyntaxNode) -> Option<Const>`. It takes a copy of the node."""


@final
class AstStatic:
    """`syntax::ast::Static`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstStatic | None:
        """`AstNode::cast(syntax: SyntaxNode) -> Option<Static>`. It takes a copy of the node."""


@final
class AstRangeExpr:
    """`syntax::ast::RangeExpr`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstRangeExpr | None:
        """`AstNode::cast(syntax: SyntaxNode) -> Option<RangeExpr>`. It takes a copy of the node."""

    def start(self) -> AstExpr | None:
        """`fn start(&self) -> Option<Expr>`"""

    def end(self) -> AstExpr | None:
        """`fn end(&self) -> Option<Expr>`"""

    def op_token(self) -> SyntaxToken | None:
        """`RangeItem::op_token(&self) -> Option<SyntaxToken>`"""


@final
class AstRangePat:
    """`syntax::ast::RangePat`"""

    @staticmethod
    def cast(syntax: SyntaxNode) -> AstRangePat | None:
        """`AstNode::cast(syntax: SyntaxNode) -> Option<RangePat>`. It takes a copy of the node."""

    def start(self) -> AstPat | None:
        """`fn start(&self) -> Option<Pat>`"""

    def end(self) -> AstPat | None:
        """`fn end(&self) -> Option<Pat>`"""

    def op_token(self) -> SyntaxToken | None:
        """`RangeItem::op_token(&self) -> Option<SyntaxToken>`"""

@final
class InRealFileSyntaxNode:
    """`hir_expand::InRealFile<SyntaxNode>`"""

    @property
    def file_id(self) -> EditionedFileId:
        """`pub file_id: EditionedFileId`"""

    @property
    def value(self) -> SyntaxNode:
        """`pub value: SyntaxNode`, lent by the receiver."""

@final
class AstString:
    """`syntax::ast::String`"""

    def value(self) -> str:
        """`fn value(&self) -> Result<Cow<'_, str>, EscapeError>`; an invalid literal raises ValueError."""

    @staticmethod
    def cast(syntax: SyntaxToken) -> AstString | None:
        """`AstToken::cast(syntax: SyntaxToken) -> Option<String>`. It takes a copy."""
