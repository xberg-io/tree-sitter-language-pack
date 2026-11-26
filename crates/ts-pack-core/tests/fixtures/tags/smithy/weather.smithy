$version: "2"

namespace example.weather

use aws.protocols#restJson1

@restJson1
service Weather {
    version: "2006-03-01"
    resources: [City]
    operations: [GetCurrentTime]
}

resource City {
    identifiers: { cityId: CityId }
    read: GetCity
}

@readonly
operation GetCity {
    input: GetCityInput
    output: GetCityOutput
}

operation GetCurrentTime {
    output := {
        time: Timestamp
    }
}

@pattern("^[A-Za-z0-9 ]+$")
string CityId

structure GetCityInput {
    @required
    cityId: CityId
}

structure GetCityOutput {
    name: String
}

enum Unit {
    CELSIUS
    FAHRENHEIT
}

list CityList {
    member: CityId
}
