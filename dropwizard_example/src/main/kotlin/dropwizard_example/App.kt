/*
 * This Kotlin source file was generated by the Gradle 'init' task.
 */
package dropwizard_example

class App {
    val greeting: String
        get() {
            return "Hello world."
        }
}

fun main(args: Array<String>) {
    HelloWorldApplication().run(*args)
}
