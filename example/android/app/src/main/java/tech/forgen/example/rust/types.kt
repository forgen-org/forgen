import kotlinx.serialization.*

@Serializable
sealed class Signal {
	@Serializable
	@SerialName("Command")
	data class command(val content: Command): Signal()
	@Serializable
	@SerialName("Event")
	data class event(val content: Event): Signal()
}

@Serializable
enum class Command(val string: String) {
	@SerialName("Start")
	Start("Start"),
	@SerialName("Stop")
	Stop("Stop"),
}

@Serializable
enum class Event(val string: String) {
	@SerialName("Started")
	Started("Started"),
	@SerialName("Stopped")
	Stopped("Stopped"),
}

