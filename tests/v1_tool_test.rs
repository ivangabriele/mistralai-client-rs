use mistralai_client::v1::client::Client;

trait _Trait: Send {}
struct _Foo {
    _dummy: Client,
}
impl _Trait for _Foo {}
