import Foundation

struct Layer : Composer {
    let uuid = NSUUID()
    var name:String?
    var description:String?
    var instrument:Instrument
    var melodies:[Melody]
    var dynamics:[Double]

    func compose() {
        for melody in melodies {
	    melody.compose()
	}
    }
}
