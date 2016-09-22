import Foundation

struct Melody : Composer {
    let uuid = NSUUID()
    var name:String?
    var measures:[Measure]
    var scale:[Int]?

    func compose() {
        for measure in measures {
	    measure.compose()
	}
    }
}
