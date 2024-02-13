//     fun findInternalPort(dockerFile: File): Int {
//         log("Finding internal docker port for docker file $dockerFile", LogType.DEBUG)
//         return dockerFile.readText().substringAfter("EXPOSE ").substringBefore("\n").toInt()
//     }