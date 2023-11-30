use proc_macro::TokenStream;
use quote::quote;
use syn;

#[proc_macro_derive(PizzaStore)]
pub fn duck_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_pizza_store_macro_derive(&ast)
}

fn impl_pizza_store_macro_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl PizzaStore for #name {
            fn order_pizza(&self, pizza_type: &str) -> Option<Box<dyn Pizza>> {
                let pizza = self.create_pizza(pizza_type);
                if let Some(mut pizza) = pizza {
                    pizza.prepare();
                    pizza.bake();
                    pizza.cut();
                    pizza.package();
                    Some(pizza)
                } else {
                    None
                }
            }
        }
    };
    gen.into()
}
