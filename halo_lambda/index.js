// HALO Oracle Serverless Lambda handler
// This Lambda function provides two API methods via API Gateway:
// - GET  /readings : Returns all reading items from DynamoDB.
// - POST /readings : Stores a new reading item in DynamoDB.

// The function uses the AWS SDK v3 for DynamoDB with the DocumentClient
// to handle marshalling between native JavaScript types and DynamoDB item
// structures automatically. The target DynamoDB table name can be
// configured via the TABLE_NAME environment variable; otherwise it
// defaults to "HaloReadings".

const { DynamoDBClient } = require("@aws-sdk/client-dynamodb");
const {
  DynamoDBDocumentClient,
  ScanCommand,
  PutCommand,
} = require("@aws-sdk/lib-dynamodb");

// Create low‑level and Document clients. The region defaults to a
// real AWS region even if someone left a placeholder like
// "MY_AWS_REGION" in their environment variables.
const resolvedRegion = (() => {
  const candidate =
    process.env.AWS_REGION || process.env.AWS_DEFAULT_REGION || "";

  // Treat obvious placeholders or malformed regions as invalid and fall back.
  // A valid AWS region looks like "us-east-1", "eu-west-3", "us-gov-west-1",
  // etc. Allow multiple dash-delimited fragments before the trailing number so
  // GovCloud/ISO/CN regions are accepted instead of misflagged as malformed.
  const isValidRegion = /^[a-z]{2}(?:-[a-z]+)+-\d+$/.test(candidate);
  if (candidate && candidate !== "MY_AWS_REGION" && isValidRegion) return candidate;

  return "us-east-1"; // safe fallback so deployments don't fail on placeholders
})();

const ddbClient = new DynamoDBClient({ region: resolvedRegion });
const ddbDocClient = DynamoDBDocumentClient.from(ddbClient);

exports.handler = async (event) => {
  const tableName = process.env.TABLE_NAME || "HaloReadings";

  try {
    // Inspect the HTTP method from the event (when invoked via API Gateway)
    const method = (event.httpMethod || event.method || "GET").toUpperCase();

    // Handle GET requests to list all readings
    if (method === "GET") {
      const data = await ddbDocClient.send(
        new ScanCommand({ TableName: tableName })
      );
      return {
        statusCode: 200,
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify(data.Items || []),
      };
    }

    // Handle POST requests to create a new reading entry
    if (method === "POST") {
      // Parse the JSON body from the request
      let item;
      try {
        item = JSON.parse(event.body || "{}");
      } catch (parseErr) {
        return {
          statusCode: 400,
          headers: { "Content-Type": "application/json" },
          body: JSON.stringify({ message: "Invalid JSON payload" }),
        };
      }

      // Write the item to DynamoDB
      await ddbDocClient.send(
        new PutCommand({ TableName: tableName, Item: item })
      );
      return {
        statusCode: 201,
        headers: { "Content-Type": "application/json" },
        body: JSON.stringify({ message: "Reading stored", item }),
      };
    }

    // Method not allowed
    return {
      statusCode: 405,
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ message: "Method not allowed" }),
    };
  } catch (err) {
    console.error("Error handling request", err);
    return {
      statusCode: 500,
      headers: { "Content-Type": "application/json" },
      body: JSON.stringify({ message: err.message || "Internal server error" }),
    };
  }
};
