import Vapor

let drop = Droplet()

drop.get("/hello") { _ in
    return "hello music"
}

drop.serve();
