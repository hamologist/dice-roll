import { APIGatewayProxyEvent, APIGatewayProxyResult } from 'aws-lambda';
import { isRollPayload } from './models/payload';
import { payloadProcessor } from './processors/payload-processor';

export const handler = async (event: APIGatewayProxyEvent): Promise<APIGatewayProxyResult> => {
    if (event.body === null) {
        return {
            statusCode: 400,
            body: 'Empty dice roll payload',
        };
    }

    let body;
    try {
        body = JSON.parse(event.body);
    } catch (error) {
        return {
            statusCode: 400,
            body: 'Malformed JSON payload provided',
        };
    }

    if (!isRollPayload(body)) {
        return {
            statusCode: 400,
            body: 'Invalid JSON payload provided',
        };
    }

    let rollResponse;
    try {
        rollResponse = payloadProcessor(body);
    } catch (error) {
        console.log(error);
        return {
            statusCode: 500,
            body: 'Failed to process roll payload',
        };
    }
    return {
        statusCode: 200,
        body: JSON.stringify(rollResponse),
    };
}
