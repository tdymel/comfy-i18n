use comfy_i18n_ast::Ast;

pub struct Context {
    pub name: String,
    pub ast: Ast,
}

pub struct Translation {
    pub context_name: String,
    pub ast: Ast,
}
