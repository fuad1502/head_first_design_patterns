use proc_macro::TokenStream;
use quote::quote;

use syn;

#[proc_macro_derive(Duck)]
pub fn duck_macro_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_duck_macro_derive(&ast)
}

fn impl_duck_macro_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl Duck for #name {
            fn display(&self) {
                println!("I am a {}! 🦆", stringify!(#name));
            }
            fn quack(&self) {
                self.quack_behaviour.quack();
            }
            fn fly(&self) {
                self.fly_behaviour.fly();
            }
            fn set_quack_behaviour(&mut self, quack_behaviour: Box<dyn QuackBehaviour>) {
                self.quack_behaviour = quack_behaviour;
            }
            fn set_fly_behaviour(&mut self, fly_behaviour: Box<dyn FlyBehaviour>) {
                self.fly_behaviour = fly_behaviour;
            }
        }
    };
    gen.into()
}
