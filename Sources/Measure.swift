struct Measure : Composer {
    var notes:[Note]
    var meter:[Int]

    func compose() {
        for note in notes {
	    note.compose()
	}
    }
}
