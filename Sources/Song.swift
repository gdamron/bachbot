import Foundation

struct Song : Composer {
    let uuid = NSUUID()
    var name:String
    var description:String?
    var author:String?
    var duration:Double
    var sections:[Section]

    func compose() {
        for section in sections {
	    section.compose()
	}
    }
}
