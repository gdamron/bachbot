import Foundation

struct Section : Composer {
    let uuid = NSUUID()
    var name:String
    var description:String?
    var duration:Double
    var tempo:Double
    var layers:[Layer]

    func compose() {
        for layer in layers {
	    layer.compose()
	}
    }
}
