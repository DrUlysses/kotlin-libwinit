import dr.ulysses.window
import uniffi.libwinit.WindowEvent

fun main() {
    window(
        name = "Window",
        height = 1024u,
        width = 480u
    ) { event ->
        when (event) {
            is WindowEvent.Resized -> {
                println("Window resized to ${event.width}x${event.height}")
            }
            WindowEvent.Destroyed -> {
                println("Window destroyed")
            }
            WindowEvent.CloseRequested -> {
                println("Window close requested")
            }
            is WindowEvent.Focused -> {
                println("Window focused: ${event.focused}")
            }
            WindowEvent.KeyboardInput -> {
                println("Keyboard input received")
            }
            WindowEvent.MouseInput -> {
                println("Mouse input received")
            }
            WindowEvent.RedrawRequested -> {
                // Redraw is handled internally, no need to log
            }
            else -> {
                // Handle all other events
                println("Other event: ${event::class.simpleName}")
            }
        }
    }
}
