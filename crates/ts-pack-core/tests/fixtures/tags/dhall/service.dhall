let Prelude = ./Prelude/package.dhall

let Service = { name : Text, port : Natural }

let defaultPort = 8080

let mkService =
      \(name : Text) ->
        let port = defaultPort
        in  { name, port } : Service

in  Prelude.List.map Text Service mkService [ "api", "worker" ]
