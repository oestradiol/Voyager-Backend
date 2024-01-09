package studio.pinkcloud.voyager.utils

import io.sentry.context.Context
import io.sentry.event.BreadcrumbBuilder
import io.sentry.event.UserBuilder
import io.sentry.Sentry
import io.sentry.SentryClient
import io.sentry.SentryClientFactory

class Sentry {
    private lateinit var sentry: SentryClient

    fun main(vararg args: String) {
        /*
         It is recommended that you use the DSN detection system, which
         will check the environment variable "SENTRY_DSN", the Java
         System Property "sentry.dsn", or the "sentry.properties" file
         in your classpath. This makes it easier to provide and adjust
         your DSN without needing to change your code. See the configuration
         page for more information.

         For example, using an environment variable

         export SENTRY_DSN="YOUR-GLITCHTIP-DSN-HERE"
         */
        Sentry.init()

        // You can also manually provide the DSN to the ``init`` method.
        Sentry.init("https://0a92da9a2ecb4ba6b84f8feea3ed31fb@sentry.pinkcloud.studio/3")

        /*
         It is possible to go around the static ``Sentry`` API, which means
         you are responsible for making the SentryClient instance available
         to your code.
         */
        sentry = SentryClientFactory.sentryClient()

        val sentry = studio.pinkcloud.voyager.utils.Sentry()
        sentry.logWithStaticAPI()
        sentry.logWithInstanceAPI()
    }

    /**
     * An example method that throws an exception.
     */
    fun unsafeMethod() {
        throw UnsupportedOperationException("You shouldn't call this!")
    }

    /**
     * Examples using the (recommended) static API.
     */
    fun logWithStaticAPI() {
        // Note that all fields set on the context are optional. Context data is copied onto
        // all future events in the current context (until the context is cleared).

        // Record a breadcrumb in the current context. By default the last 100 breadcrumbs are kept.
        Sentry.getContext().recordBreadcrumb(
            BreadcrumbBuilder().setMessage("User made an action").build()
        )

        // Set the user in the current context.
        Sentry.getContext().setUser(
            UserBuilder().setEmail("hello@sentry.io").build()
        )

        // Add extra data to future events in this context.
        Sentry.getContext().addExtra("extra", "thing")

        // Add an additional tag to future events in this context.
        Sentry.getContext().addTag("tagName", "tagValue")

        /*
         This sends a simple event to Sentry using the statically stored instance
         that was created in the ``main`` method.
         */
        Sentry.capture("This is a test")

        try {
            unsafeMethod()
        } catch (e: Exception) {
            // This sends an exception event to Sentry using the statically stored instance
            // that was created in the ``main`` method.
            Sentry.capture(e)
        }
    }

    /**
     * Examples that use the SentryClient instance directly.
     */
    fun logWithInstanceAPI() {
        // Retrieve the current context.
        val context: Context = sentry.getContext()

        // Record a breadcrumb in the current context. By default the last 100 breadcrumbs are kept.
        context.recordBreadcrumb(BreadcrumbBuilder().setMessage("User made an action").build())

        // Set the user in the current context.
        context.setUser(UserBuilder().setEmail("hello@sentry.io").build())

        // This sends a simple event to Sentry.
        sentry.sendMessage("This is a test")

        try {
            unsafeMethod()
        } catch (e: Exception) {
            // This sends an exception event to Sentry.
            sentry.sendException(e)
        }
    }
}
