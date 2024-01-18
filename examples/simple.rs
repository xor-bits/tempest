use xhtml_macro::view;

//

fn main() {
    let html = view! {
        <div>
            <a>Hello</a>
            <button attrs/>
        </div>
    };
    println!();
}
