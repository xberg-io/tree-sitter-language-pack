import regex

schema Container:
    name: str
    image: str
    port: int = 8080

    check:
        port > 0

schema App:
    name: str
    containers: [Container]

protocol Named:
    name: str

rule HasName:
    len(name) > 0

replicas = 3

app = App {
    name = "web"
    containers = [Container {name = "web", image = "nginx"}]
}
