package studio.pinkcloud.voyager.routing.annotations

@Retention(AnnotationRetention.SOURCE)
@Target(AnnotationTarget.EXPRESSION)
// will be used for the routing system to check if the user is logged in or not
// other annotationms will be made to make sure that the user is an admin or has a certain permission or is a client of a project ect
annotation class LoggedIn 