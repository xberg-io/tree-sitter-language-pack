import "@typespec/http";

using TypeSpec.Http;

@service
namespace PetStore;

enum Species {
  dog,
  cat,
}

model Pet {
  id: int32;
  name: string;
  species: Species;
}

union PetOrError {
  pet: Pet,
  error: Error,
}

@error
model Error {
  code: int32;
}

scalar PetId extends string;

alias PetList = Pet[];

@route("/pets")
interface Pets {
  list(): PetList;
  @get read(@path id: PetId): Pet | Error;
}

op ping(): void;
