struct Note : Composer {
    var scaleDegree:[Int]
    var duration:Double
    var onset:Int
    var envelope:[Double]?

    func compose() {
        scaleDegree = random() % 12
    }
}
