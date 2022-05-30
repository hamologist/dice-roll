import axios from 'axios';
import { rollFromDiscordProcessor } from './processors/roll-from-discord-processor';

interface Data {
    id: string;
    name: string;
    options: Array<{
        name: string;
        value: string;
    }>;
}

export interface VerifiedInteraction {
    data: Data
    type: number;
    token: string;
    applicationId: string;
}

export const handler = async (
    interaction: VerifiedInteraction,
): Promise<void> => {
    const response = rollFromDiscordProcessor(interaction.data.options[0].value);

    if (!response.flags) {
        await axios.patch(
            `https://discord.com/api/v8/webhooks/${interaction.applicationId}/${interaction.token}/messages/@original`,
            { content: response.content }
        );
    } else {
        await axios.delete(
            `https://discord.com/api/v8/webhooks/${interaction.applicationId}/${interaction.token}/messages/@original`
        );
        await axios.post(
            `https://discord.com/api/v8/webhooks/${interaction.applicationId}/${interaction.token}`,
            response,
        );
    }
};
