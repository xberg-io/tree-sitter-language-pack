@0xdbb9ad1f14bf0b36;

using Json = import "json.capnp";

const defaultName :Text = "anonymous";

struct Person {
  id @0 :UInt32;
  name @1 :Text;
  phones @2 :List(PhoneNumber);

  struct PhoneNumber {
    number @0 :Text;
    type @1 :Type;

    enum Type {
      mobile @0;
      home @1;
    }
  }
}

interface Directory {
  lookup @0 (name :Text) -> (person :Person);
}
