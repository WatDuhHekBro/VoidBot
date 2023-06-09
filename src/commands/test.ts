import { ChatInputCommandInteraction } from "discord.js";
import {
	AudioPlayerStatus,
	getVoiceConnection,
	joinVoiceChannel,
	createAudioPlayer,
	createAudioResource,
	StreamType,
	generateDependencyReport,
	VoiceConnectionStatus,
} from "@discordjs/voice";

console.debug(generateDependencyReport());

export async function execute(interaction: ChatInputCommandInteraction) {
	if (interaction.guild && interaction.channel) {
		joinVoiceChannel({
			channelId: interaction.channel.id,
			guildId: interaction.guild.id,
			adapterCreator: interaction.guild.voiceAdapterCreator,
		});

		const connection = getVoiceConnection(interaction.guild.id);

		if (connection) {
			connection.on(VoiceConnectionStatus.Ready, (oldState, newState) => {
				setTimeout(() => {
					const player = createAudioPlayer();
					const resource = createAudioResource(
						"audio/dark-hour.ogg",
						{
							inputType: StreamType.OggOpus,
						}
					);
					connection.subscribe(player);
					player.play(resource);
					player.on(AudioPlayerStatus.Idle, () => {
						connection.destroy();
					});
					player.on("stateChange", () =>
						console.debug("STATE CHANGE")
					);

					setInterval(() => {
						if (player.state.status === AudioPlayerStatus.Playing) {
							console.debug("attempt");
							const a = player.state;
							a.playbackDuration = 0;
							player.state = a;
						}
					}, 5_000);
				}, 1_000);
			});
		}
	}
	await interaction.reply({ content: "test", ephemeral: true });
}
