package dr.ulysses

import uniffi.libwinit.Window
import uniffi.libwinit.WindowEvent
import kotlinx.coroutines.*
import kotlin.time.Duration.Companion.milliseconds

/**
 * DSL function to create and run a window with event handling using coroutines.
 *
 * @param name The window title
 * @param width The window width in pixels
 * @param height The window height in pixels
 * @param eventHandler Lambda that receives WindowEvent and handles it
 */
fun window(
    name: String,
    width: UInt,
    height: UInt,
    eventHandler: (WindowEvent) -> Unit
) = runBlocking {
    val window = Window(title = name, width = width, height = height)
    
    // Start the window event loop in background thread
    window.start()
    
    // Poll for events in a coroutine
    val eventJob = launch(Dispatchers.Default) {
        while (window.isRunning()) {
            // Poll for events
            val event = window.pollEvent()
            
            if (event != null) {
                // Invoke user's event handler
                eventHandler(event)
                
                // Exit if close was requested
                if (event is WindowEvent.CloseRequested) {
                    window.stop()
                    break
                }
            } else {
                // No events available, wait a bit before polling again
                delay(16.milliseconds) // ~60 FPS polling rate
            }
        }
    }
    
    // Wait for the event loop to finish
    eventJob.join()
}
