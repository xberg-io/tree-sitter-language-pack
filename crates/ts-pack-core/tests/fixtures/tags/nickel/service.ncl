let Port = std.contract.from_predicate (fun p => p > 0 && p < 65536) in
let make_service = fun name port =>
  {
    name = name,
    port | Port = port,
    image = "ghcr.io/example/%{name}",
  }
in
let Service = {
  name | String,
  port | Port,
  image | String,
}
in
{
  api | Service = make_service "api" 8080,
  worker | Service = make_service "worker" 9090,
}
