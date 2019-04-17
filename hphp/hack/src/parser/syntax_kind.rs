/**
 * Copyright (c) 2016, Facebook, Inc.
 * All rights reserved.
 *
 * This source code is licensed under the MIT license found in the
 * LICENSE file in the "hack" directory of this source tree. An additional
 * directory.
 *
 **
 *
 * THIS FILE IS @generated; DO NOT EDIT IT
 * To regenerate this file, run
 *
 *   buck run //hphp/hack/src:generate_full_fidelity
 *
 **
 *
 */

#[derive(Debug, Copy, Clone)]
pub enum SyntaxKind {
    Missing,
    Token,
    SyntaxList,
    EndOfFile,
    Script,
    QualifiedName,
    SimpleTypeSpecifier,
    LiteralExpression,
    PrefixedStringExpression,
    VariableExpression,
    PipeVariableExpression,
    FileAttributeSpecification,
    EnumDeclaration,
    Enumerator,
    RecordDeclaration,
    RecordField,
    AliasDeclaration,
    PropertyDeclaration,
    PropertyDeclarator,
    NamespaceDeclaration,
    NamespaceBody,
    NamespaceEmptyBody,
    NamespaceUseDeclaration,
    NamespaceGroupUseDeclaration,
    NamespaceUseClause,
    FunctionDeclaration,
    FunctionDeclarationHeader,
    WhereClause,
    WhereConstraint,
    MethodishDeclaration,
    MethodishTraitResolution,
    ClassishDeclaration,
    ClassishBody,
    TraitUsePrecedenceItem,
    TraitUseAliasItem,
    TraitUseConflictResolution,
    TraitUse,
    RequireClause,
    ConstDeclaration,
    ConstantDeclarator,
    TypeConstDeclaration,
    DecoratedExpression,
    ParameterDeclaration,
    VariadicParameter,
    AttributeSpecification,
    InclusionExpression,
    InclusionDirective,
    CompoundStatement,
    AlternateLoopStatement,
    ExpressionStatement,
    MarkupSection,
    MarkupSuffix,
    UnsetStatement,
    LetStatement,
    UsingStatementBlockScoped,
    UsingStatementFunctionScoped,
    DeclareDirectiveStatement,
    DeclareBlockStatement,
    WhileStatement,
    IfStatement,
    ElseifClause,
    ElseClause,
    AlternateIfStatement,
    AlternateElseifClause,
    AlternateElseClause,
    TryStatement,
    CatchClause,
    FinallyClause,
    DoStatement,
    ForStatement,
    ForeachStatement,
    SwitchStatement,
    AlternateSwitchStatement,
    SwitchSection,
    SwitchFallthrough,
    CaseLabel,
    DefaultLabel,
    ReturnStatement,
    GotoLabel,
    GotoStatement,
    ThrowStatement,
    BreakStatement,
    ContinueStatement,
    EchoStatement,
    ConcurrentStatement,
    SimpleInitializer,
    AnonymousClass,
    AnonymousFunction,
    Php7AnonymousFunction,
    AnonymousFunctionUseClause,
    LambdaExpression,
    LambdaSignature,
    CastExpression,
    ScopeResolutionExpression,
    MemberSelectionExpression,
    SafeMemberSelectionExpression,
    EmbeddedMemberSelectionExpression,
    YieldExpression,
    YieldFromExpression,
    PrefixUnaryExpression,
    PostfixUnaryExpression,
    BinaryExpression,
    InstanceofExpression,
    IsExpression,
    AsExpression,
    NullableAsExpression,
    ConditionalExpression,
    EvalExpression,
    EmptyExpression,
    DefineExpression,
    HaltCompilerExpression,
    IssetExpression,
    FunctionCallExpression,
    ParenthesizedExpression,
    BracedExpression,
    EmbeddedBracedExpression,
    ListExpression,
    CollectionLiteralExpression,
    ObjectCreationExpression,
    ConstructorCall,
    RecordCreationExpression,
    ArrayCreationExpression,
    ArrayIntrinsicExpression,
    DarrayIntrinsicExpression,
    DictionaryIntrinsicExpression,
    KeysetIntrinsicExpression,
    VarrayIntrinsicExpression,
    VectorIntrinsicExpression,
    ElementInitializer,
    SubscriptExpression,
    EmbeddedSubscriptExpression,
    AwaitableCreationExpression,
    XHPChildrenDeclaration,
    XHPChildrenParenthesizedList,
    XHPCategoryDeclaration,
    XHPEnumType,
    XHPRequired,
    XHPClassAttributeDeclaration,
    XHPClassAttribute,
    XHPSimpleClassAttribute,
    XHPSimpleAttribute,
    XHPSpreadAttribute,
    XHPOpen,
    XHPExpression,
    XHPClose,
    TypeConstant,
    VectorTypeSpecifier,
    KeysetTypeSpecifier,
    TupleTypeExplicitSpecifier,
    VarrayTypeSpecifier,
    VectorArrayTypeSpecifier,
    TypeParameter,
    TypeConstraint,
    DarrayTypeSpecifier,
    MapArrayTypeSpecifier,
    DictionaryTypeSpecifier,
    ClosureTypeSpecifier,
    ClosureParameterTypeSpecifier,
    ClassnameTypeSpecifier,
    FieldSpecifier,
    FieldInitializer,
    ShapeTypeSpecifier,
    ShapeExpression,
    TupleExpression,
    GenericTypeSpecifier,
    NullableTypeSpecifier,
    LikeTypeSpecifier,
    SoftTypeSpecifier,
    ReifiedTypeArgument,
    TypeArguments,
    TypeParameters,
    TupleTypeSpecifier,
    ErrorSyntax,
    ListItem,
    PocketAtomExpression,
    PocketIdentifierExpression,
    PocketAtomMappingDeclaration,
    PocketEnumDeclaration,
    PocketFieldTypeExprDeclaration,
    PocketFieldTypeDeclaration,
    PocketMappingIdDeclaration,
    PocketMappingTypeDeclaration,

}

impl SyntaxKind {
    pub fn to_string(&self) -> &str {
        match self {
            SyntaxKind::SyntaxList => "list",
            SyntaxKind::Missing => "missing",
            SyntaxKind::Token => "token",
            SyntaxKind::EndOfFile                         => "end_of_file",
            SyntaxKind::Script                            => "script",
            SyntaxKind::QualifiedName                     => "qualified_name",
            SyntaxKind::SimpleTypeSpecifier               => "simple_type_specifier",
            SyntaxKind::LiteralExpression                 => "literal",
            SyntaxKind::PrefixedStringExpression          => "prefixed_string",
            SyntaxKind::VariableExpression                => "variable",
            SyntaxKind::PipeVariableExpression            => "pipe_variable",
            SyntaxKind::FileAttributeSpecification        => "file_attribute_specification",
            SyntaxKind::EnumDeclaration                   => "enum_declaration",
            SyntaxKind::Enumerator                        => "enumerator",
            SyntaxKind::RecordDeclaration                 => "record_declaration",
            SyntaxKind::RecordField                       => "record_field",
            SyntaxKind::AliasDeclaration                  => "alias_declaration",
            SyntaxKind::PropertyDeclaration               => "property_declaration",
            SyntaxKind::PropertyDeclarator                => "property_declarator",
            SyntaxKind::NamespaceDeclaration              => "namespace_declaration",
            SyntaxKind::NamespaceBody                     => "namespace_body",
            SyntaxKind::NamespaceEmptyBody                => "namespace_empty_body",
            SyntaxKind::NamespaceUseDeclaration           => "namespace_use_declaration",
            SyntaxKind::NamespaceGroupUseDeclaration      => "namespace_group_use_declaration",
            SyntaxKind::NamespaceUseClause                => "namespace_use_clause",
            SyntaxKind::FunctionDeclaration               => "function_declaration",
            SyntaxKind::FunctionDeclarationHeader         => "function_declaration_header",
            SyntaxKind::WhereClause                       => "where_clause",
            SyntaxKind::WhereConstraint                   => "where_constraint",
            SyntaxKind::MethodishDeclaration              => "methodish_declaration",
            SyntaxKind::MethodishTraitResolution          => "methodish_trait_resolution",
            SyntaxKind::ClassishDeclaration               => "classish_declaration",
            SyntaxKind::ClassishBody                      => "classish_body",
            SyntaxKind::TraitUsePrecedenceItem            => "trait_use_precedence_item",
            SyntaxKind::TraitUseAliasItem                 => "trait_use_alias_item",
            SyntaxKind::TraitUseConflictResolution        => "trait_use_conflict_resolution",
            SyntaxKind::TraitUse                          => "trait_use",
            SyntaxKind::RequireClause                     => "require_clause",
            SyntaxKind::ConstDeclaration                  => "const_declaration",
            SyntaxKind::ConstantDeclarator                => "constant_declarator",
            SyntaxKind::TypeConstDeclaration              => "type_const_declaration",
            SyntaxKind::DecoratedExpression               => "decorated_expression",
            SyntaxKind::ParameterDeclaration              => "parameter_declaration",
            SyntaxKind::VariadicParameter                 => "variadic_parameter",
            SyntaxKind::AttributeSpecification            => "attribute_specification",
            SyntaxKind::InclusionExpression               => "inclusion_expression",
            SyntaxKind::InclusionDirective                => "inclusion_directive",
            SyntaxKind::CompoundStatement                 => "compound_statement",
            SyntaxKind::AlternateLoopStatement            => "alternate_loop_statement",
            SyntaxKind::ExpressionStatement               => "expression_statement",
            SyntaxKind::MarkupSection                     => "markup_section",
            SyntaxKind::MarkupSuffix                      => "markup_suffix",
            SyntaxKind::UnsetStatement                    => "unset_statement",
            SyntaxKind::LetStatement                      => "let_statement",
            SyntaxKind::UsingStatementBlockScoped         => "using_statement_block_scoped",
            SyntaxKind::UsingStatementFunctionScoped      => "using_statement_function_scoped",
            SyntaxKind::DeclareDirectiveStatement         => "declare_directive_statement",
            SyntaxKind::DeclareBlockStatement             => "declare_block_statement",
            SyntaxKind::WhileStatement                    => "while_statement",
            SyntaxKind::IfStatement                       => "if_statement",
            SyntaxKind::ElseifClause                      => "elseif_clause",
            SyntaxKind::ElseClause                        => "else_clause",
            SyntaxKind::AlternateIfStatement              => "alternate_if_statement",
            SyntaxKind::AlternateElseifClause             => "alternate_elseif_clause",
            SyntaxKind::AlternateElseClause               => "alternate_else_clause",
            SyntaxKind::TryStatement                      => "try_statement",
            SyntaxKind::CatchClause                       => "catch_clause",
            SyntaxKind::FinallyClause                     => "finally_clause",
            SyntaxKind::DoStatement                       => "do_statement",
            SyntaxKind::ForStatement                      => "for_statement",
            SyntaxKind::ForeachStatement                  => "foreach_statement",
            SyntaxKind::SwitchStatement                   => "switch_statement",
            SyntaxKind::AlternateSwitchStatement          => "alternate_switch_statement",
            SyntaxKind::SwitchSection                     => "switch_section",
            SyntaxKind::SwitchFallthrough                 => "switch_fallthrough",
            SyntaxKind::CaseLabel                         => "case_label",
            SyntaxKind::DefaultLabel                      => "default_label",
            SyntaxKind::ReturnStatement                   => "return_statement",
            SyntaxKind::GotoLabel                         => "goto_label",
            SyntaxKind::GotoStatement                     => "goto_statement",
            SyntaxKind::ThrowStatement                    => "throw_statement",
            SyntaxKind::BreakStatement                    => "break_statement",
            SyntaxKind::ContinueStatement                 => "continue_statement",
            SyntaxKind::EchoStatement                     => "echo_statement",
            SyntaxKind::ConcurrentStatement               => "concurrent_statement",
            SyntaxKind::SimpleInitializer                 => "simple_initializer",
            SyntaxKind::AnonymousClass                    => "anonymous_class",
            SyntaxKind::AnonymousFunction                 => "anonymous_function",
            SyntaxKind::Php7AnonymousFunction             => "php7_anonymous_function",
            SyntaxKind::AnonymousFunctionUseClause        => "anonymous_function_use_clause",
            SyntaxKind::LambdaExpression                  => "lambda_expression",
            SyntaxKind::LambdaSignature                   => "lambda_signature",
            SyntaxKind::CastExpression                    => "cast_expression",
            SyntaxKind::ScopeResolutionExpression         => "scope_resolution_expression",
            SyntaxKind::MemberSelectionExpression         => "member_selection_expression",
            SyntaxKind::SafeMemberSelectionExpression     => "safe_member_selection_expression",
            SyntaxKind::EmbeddedMemberSelectionExpression => "embedded_member_selection_expression",
            SyntaxKind::YieldExpression                   => "yield_expression",
            SyntaxKind::YieldFromExpression               => "yield_from_expression",
            SyntaxKind::PrefixUnaryExpression             => "prefix_unary_expression",
            SyntaxKind::PostfixUnaryExpression            => "postfix_unary_expression",
            SyntaxKind::BinaryExpression                  => "binary_expression",
            SyntaxKind::InstanceofExpression              => "instanceof_expression",
            SyntaxKind::IsExpression                      => "is_expression",
            SyntaxKind::AsExpression                      => "as_expression",
            SyntaxKind::NullableAsExpression              => "nullable_as_expression",
            SyntaxKind::ConditionalExpression             => "conditional_expression",
            SyntaxKind::EvalExpression                    => "eval_expression",
            SyntaxKind::EmptyExpression                   => "empty_expression",
            SyntaxKind::DefineExpression                  => "define_expression",
            SyntaxKind::HaltCompilerExpression            => "halt_compiler_expression",
            SyntaxKind::IssetExpression                   => "isset_expression",
            SyntaxKind::FunctionCallExpression            => "function_call_expression",
            SyntaxKind::ParenthesizedExpression           => "parenthesized_expression",
            SyntaxKind::BracedExpression                  => "braced_expression",
            SyntaxKind::EmbeddedBracedExpression          => "embedded_braced_expression",
            SyntaxKind::ListExpression                    => "list_expression",
            SyntaxKind::CollectionLiteralExpression       => "collection_literal_expression",
            SyntaxKind::ObjectCreationExpression          => "object_creation_expression",
            SyntaxKind::ConstructorCall                   => "constructor_call",
            SyntaxKind::RecordCreationExpression          => "record_creation_expression",
            SyntaxKind::ArrayCreationExpression           => "array_creation_expression",
            SyntaxKind::ArrayIntrinsicExpression          => "array_intrinsic_expression",
            SyntaxKind::DarrayIntrinsicExpression         => "darray_intrinsic_expression",
            SyntaxKind::DictionaryIntrinsicExpression     => "dictionary_intrinsic_expression",
            SyntaxKind::KeysetIntrinsicExpression         => "keyset_intrinsic_expression",
            SyntaxKind::VarrayIntrinsicExpression         => "varray_intrinsic_expression",
            SyntaxKind::VectorIntrinsicExpression         => "vector_intrinsic_expression",
            SyntaxKind::ElementInitializer                => "element_initializer",
            SyntaxKind::SubscriptExpression               => "subscript_expression",
            SyntaxKind::EmbeddedSubscriptExpression       => "embedded_subscript_expression",
            SyntaxKind::AwaitableCreationExpression       => "awaitable_creation_expression",
            SyntaxKind::XHPChildrenDeclaration            => "xhp_children_declaration",
            SyntaxKind::XHPChildrenParenthesizedList      => "xhp_children_parenthesized_list",
            SyntaxKind::XHPCategoryDeclaration            => "xhp_category_declaration",
            SyntaxKind::XHPEnumType                       => "xhp_enum_type",
            SyntaxKind::XHPRequired                       => "xhp_required",
            SyntaxKind::XHPClassAttributeDeclaration      => "xhp_class_attribute_declaration",
            SyntaxKind::XHPClassAttribute                 => "xhp_class_attribute",
            SyntaxKind::XHPSimpleClassAttribute           => "xhp_simple_class_attribute",
            SyntaxKind::XHPSimpleAttribute                => "xhp_simple_attribute",
            SyntaxKind::XHPSpreadAttribute                => "xhp_spread_attribute",
            SyntaxKind::XHPOpen                           => "xhp_open",
            SyntaxKind::XHPExpression                     => "xhp_expression",
            SyntaxKind::XHPClose                          => "xhp_close",
            SyntaxKind::TypeConstant                      => "type_constant",
            SyntaxKind::VectorTypeSpecifier               => "vector_type_specifier",
            SyntaxKind::KeysetTypeSpecifier               => "keyset_type_specifier",
            SyntaxKind::TupleTypeExplicitSpecifier        => "tuple_type_explicit_specifier",
            SyntaxKind::VarrayTypeSpecifier               => "varray_type_specifier",
            SyntaxKind::VectorArrayTypeSpecifier          => "vector_array_type_specifier",
            SyntaxKind::TypeParameter                     => "type_parameter",
            SyntaxKind::TypeConstraint                    => "type_constraint",
            SyntaxKind::DarrayTypeSpecifier               => "darray_type_specifier",
            SyntaxKind::MapArrayTypeSpecifier             => "map_array_type_specifier",
            SyntaxKind::DictionaryTypeSpecifier           => "dictionary_type_specifier",
            SyntaxKind::ClosureTypeSpecifier              => "closure_type_specifier",
            SyntaxKind::ClosureParameterTypeSpecifier     => "closure_parameter_type_specifier",
            SyntaxKind::ClassnameTypeSpecifier            => "classname_type_specifier",
            SyntaxKind::FieldSpecifier                    => "field_specifier",
            SyntaxKind::FieldInitializer                  => "field_initializer",
            SyntaxKind::ShapeTypeSpecifier                => "shape_type_specifier",
            SyntaxKind::ShapeExpression                   => "shape_expression",
            SyntaxKind::TupleExpression                   => "tuple_expression",
            SyntaxKind::GenericTypeSpecifier              => "generic_type_specifier",
            SyntaxKind::NullableTypeSpecifier             => "nullable_type_specifier",
            SyntaxKind::LikeTypeSpecifier                 => "like_type_specifier",
            SyntaxKind::SoftTypeSpecifier                 => "soft_type_specifier",
            SyntaxKind::ReifiedTypeArgument               => "reified_type_argument",
            SyntaxKind::TypeArguments                     => "type_arguments",
            SyntaxKind::TypeParameters                    => "type_parameters",
            SyntaxKind::TupleTypeSpecifier                => "tuple_type_specifier",
            SyntaxKind::ErrorSyntax                       => "error",
            SyntaxKind::ListItem                          => "list_item",
            SyntaxKind::PocketAtomExpression              => "pocket_atom",
            SyntaxKind::PocketIdentifierExpression        => "pocket_identifier",
            SyntaxKind::PocketAtomMappingDeclaration      => "pocket_atom_mapping",
            SyntaxKind::PocketEnumDeclaration             => "pocket_enum_declaration",
            SyntaxKind::PocketFieldTypeExprDeclaration    => "pocket_field_type_expr_declaration",
            SyntaxKind::PocketFieldTypeDeclaration        => "pocket_field_type_declaration",
            SyntaxKind::PocketMappingIdDeclaration        => "pocket_mapping_id_declaration",
            SyntaxKind::PocketMappingTypeDeclaration      => "pocket_mapping_type_declaration",
        }
    }

    pub fn ocaml_tag(&self) -> u8 {
        match self {
            SyntaxKind::Missing => 0,
            SyntaxKind::Token => 0,
            SyntaxKind::SyntaxList => 1,
            SyntaxKind::EndOfFile => 2,
            SyntaxKind::Script => 3,
            SyntaxKind::QualifiedName => 4,
            SyntaxKind::SimpleTypeSpecifier => 5,
            SyntaxKind::LiteralExpression => 6,
            SyntaxKind::PrefixedStringExpression => 7,
            SyntaxKind::VariableExpression => 8,
            SyntaxKind::PipeVariableExpression => 9,
            SyntaxKind::FileAttributeSpecification => 10,
            SyntaxKind::EnumDeclaration => 11,
            SyntaxKind::Enumerator => 12,
            SyntaxKind::RecordDeclaration => 13,
            SyntaxKind::RecordField => 14,
            SyntaxKind::AliasDeclaration => 15,
            SyntaxKind::PropertyDeclaration => 16,
            SyntaxKind::PropertyDeclarator => 17,
            SyntaxKind::NamespaceDeclaration => 18,
            SyntaxKind::NamespaceBody => 19,
            SyntaxKind::NamespaceEmptyBody => 20,
            SyntaxKind::NamespaceUseDeclaration => 21,
            SyntaxKind::NamespaceGroupUseDeclaration => 22,
            SyntaxKind::NamespaceUseClause => 23,
            SyntaxKind::FunctionDeclaration => 24,
            SyntaxKind::FunctionDeclarationHeader => 25,
            SyntaxKind::WhereClause => 26,
            SyntaxKind::WhereConstraint => 27,
            SyntaxKind::MethodishDeclaration => 28,
            SyntaxKind::MethodishTraitResolution => 29,
            SyntaxKind::ClassishDeclaration => 30,
            SyntaxKind::ClassishBody => 31,
            SyntaxKind::TraitUsePrecedenceItem => 32,
            SyntaxKind::TraitUseAliasItem => 33,
            SyntaxKind::TraitUseConflictResolution => 34,
            SyntaxKind::TraitUse => 35,
            SyntaxKind::RequireClause => 36,
            SyntaxKind::ConstDeclaration => 37,
            SyntaxKind::ConstantDeclarator => 38,
            SyntaxKind::TypeConstDeclaration => 39,
            SyntaxKind::DecoratedExpression => 40,
            SyntaxKind::ParameterDeclaration => 41,
            SyntaxKind::VariadicParameter => 42,
            SyntaxKind::AttributeSpecification => 43,
            SyntaxKind::InclusionExpression => 44,
            SyntaxKind::InclusionDirective => 45,
            SyntaxKind::CompoundStatement => 46,
            SyntaxKind::AlternateLoopStatement => 47,
            SyntaxKind::ExpressionStatement => 48,
            SyntaxKind::MarkupSection => 49,
            SyntaxKind::MarkupSuffix => 50,
            SyntaxKind::UnsetStatement => 51,
            SyntaxKind::LetStatement => 52,
            SyntaxKind::UsingStatementBlockScoped => 53,
            SyntaxKind::UsingStatementFunctionScoped => 54,
            SyntaxKind::DeclareDirectiveStatement => 55,
            SyntaxKind::DeclareBlockStatement => 56,
            SyntaxKind::WhileStatement => 57,
            SyntaxKind::IfStatement => 58,
            SyntaxKind::ElseifClause => 59,
            SyntaxKind::ElseClause => 60,
            SyntaxKind::AlternateIfStatement => 61,
            SyntaxKind::AlternateElseifClause => 62,
            SyntaxKind::AlternateElseClause => 63,
            SyntaxKind::TryStatement => 64,
            SyntaxKind::CatchClause => 65,
            SyntaxKind::FinallyClause => 66,
            SyntaxKind::DoStatement => 67,
            SyntaxKind::ForStatement => 68,
            SyntaxKind::ForeachStatement => 69,
            SyntaxKind::SwitchStatement => 70,
            SyntaxKind::AlternateSwitchStatement => 71,
            SyntaxKind::SwitchSection => 72,
            SyntaxKind::SwitchFallthrough => 73,
            SyntaxKind::CaseLabel => 74,
            SyntaxKind::DefaultLabel => 75,
            SyntaxKind::ReturnStatement => 76,
            SyntaxKind::GotoLabel => 77,
            SyntaxKind::GotoStatement => 78,
            SyntaxKind::ThrowStatement => 79,
            SyntaxKind::BreakStatement => 80,
            SyntaxKind::ContinueStatement => 81,
            SyntaxKind::EchoStatement => 82,
            SyntaxKind::ConcurrentStatement => 83,
            SyntaxKind::SimpleInitializer => 84,
            SyntaxKind::AnonymousClass => 85,
            SyntaxKind::AnonymousFunction => 86,
            SyntaxKind::Php7AnonymousFunction => 87,
            SyntaxKind::AnonymousFunctionUseClause => 88,
            SyntaxKind::LambdaExpression => 89,
            SyntaxKind::LambdaSignature => 90,
            SyntaxKind::CastExpression => 91,
            SyntaxKind::ScopeResolutionExpression => 92,
            SyntaxKind::MemberSelectionExpression => 93,
            SyntaxKind::SafeMemberSelectionExpression => 94,
            SyntaxKind::EmbeddedMemberSelectionExpression => 95,
            SyntaxKind::YieldExpression => 96,
            SyntaxKind::YieldFromExpression => 97,
            SyntaxKind::PrefixUnaryExpression => 98,
            SyntaxKind::PostfixUnaryExpression => 99,
            SyntaxKind::BinaryExpression => 100,
            SyntaxKind::InstanceofExpression => 101,
            SyntaxKind::IsExpression => 102,
            SyntaxKind::AsExpression => 103,
            SyntaxKind::NullableAsExpression => 104,
            SyntaxKind::ConditionalExpression => 105,
            SyntaxKind::EvalExpression => 106,
            SyntaxKind::EmptyExpression => 107,
            SyntaxKind::DefineExpression => 108,
            SyntaxKind::HaltCompilerExpression => 109,
            SyntaxKind::IssetExpression => 110,
            SyntaxKind::FunctionCallExpression => 111,
            SyntaxKind::ParenthesizedExpression => 112,
            SyntaxKind::BracedExpression => 113,
            SyntaxKind::EmbeddedBracedExpression => 114,
            SyntaxKind::ListExpression => 115,
            SyntaxKind::CollectionLiteralExpression => 116,
            SyntaxKind::ObjectCreationExpression => 117,
            SyntaxKind::ConstructorCall => 118,
            SyntaxKind::RecordCreationExpression => 119,
            SyntaxKind::ArrayCreationExpression => 120,
            SyntaxKind::ArrayIntrinsicExpression => 121,
            SyntaxKind::DarrayIntrinsicExpression => 122,
            SyntaxKind::DictionaryIntrinsicExpression => 123,
            SyntaxKind::KeysetIntrinsicExpression => 124,
            SyntaxKind::VarrayIntrinsicExpression => 125,
            SyntaxKind::VectorIntrinsicExpression => 126,
            SyntaxKind::ElementInitializer => 127,
            SyntaxKind::SubscriptExpression => 128,
            SyntaxKind::EmbeddedSubscriptExpression => 129,
            SyntaxKind::AwaitableCreationExpression => 130,
            SyntaxKind::XHPChildrenDeclaration => 131,
            SyntaxKind::XHPChildrenParenthesizedList => 132,
            SyntaxKind::XHPCategoryDeclaration => 133,
            SyntaxKind::XHPEnumType => 134,
            SyntaxKind::XHPRequired => 135,
            SyntaxKind::XHPClassAttributeDeclaration => 136,
            SyntaxKind::XHPClassAttribute => 137,
            SyntaxKind::XHPSimpleClassAttribute => 138,
            SyntaxKind::XHPSimpleAttribute => 139,
            SyntaxKind::XHPSpreadAttribute => 140,
            SyntaxKind::XHPOpen => 141,
            SyntaxKind::XHPExpression => 142,
            SyntaxKind::XHPClose => 143,
            SyntaxKind::TypeConstant => 144,
            SyntaxKind::VectorTypeSpecifier => 145,
            SyntaxKind::KeysetTypeSpecifier => 146,
            SyntaxKind::TupleTypeExplicitSpecifier => 147,
            SyntaxKind::VarrayTypeSpecifier => 148,
            SyntaxKind::VectorArrayTypeSpecifier => 149,
            SyntaxKind::TypeParameter => 150,
            SyntaxKind::TypeConstraint => 151,
            SyntaxKind::DarrayTypeSpecifier => 152,
            SyntaxKind::MapArrayTypeSpecifier => 153,
            SyntaxKind::DictionaryTypeSpecifier => 154,
            SyntaxKind::ClosureTypeSpecifier => 155,
            SyntaxKind::ClosureParameterTypeSpecifier => 156,
            SyntaxKind::ClassnameTypeSpecifier => 157,
            SyntaxKind::FieldSpecifier => 158,
            SyntaxKind::FieldInitializer => 159,
            SyntaxKind::ShapeTypeSpecifier => 160,
            SyntaxKind::ShapeExpression => 161,
            SyntaxKind::TupleExpression => 162,
            SyntaxKind::GenericTypeSpecifier => 163,
            SyntaxKind::NullableTypeSpecifier => 164,
            SyntaxKind::LikeTypeSpecifier => 165,
            SyntaxKind::SoftTypeSpecifier => 166,
            SyntaxKind::ReifiedTypeArgument => 167,
            SyntaxKind::TypeArguments => 168,
            SyntaxKind::TypeParameters => 169,
            SyntaxKind::TupleTypeSpecifier => 170,
            SyntaxKind::ErrorSyntax => 171,
            SyntaxKind::ListItem => 172,
            SyntaxKind::PocketAtomExpression => 173,
            SyntaxKind::PocketIdentifierExpression => 174,
            SyntaxKind::PocketAtomMappingDeclaration => 175,
            SyntaxKind::PocketEnumDeclaration => 176,
            SyntaxKind::PocketFieldTypeExprDeclaration => 177,
            SyntaxKind::PocketFieldTypeDeclaration => 178,
            SyntaxKind::PocketMappingIdDeclaration => 179,
            SyntaxKind::PocketMappingTypeDeclaration => 180,
        }
    }
}