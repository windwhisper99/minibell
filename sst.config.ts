/// <reference path="./.sst/platform/config.d.ts" />

export default $config({
  app(input) {
    return {
      name: "minibell",
      removal: input?.stage === "production" ? "retain" : "remove",
      home: "aws",
    };
  },
  async run() {
    const table = new sst.aws.Dynamo("Primary", {
      fields: {
        PK: "string",
        SK: "string",
        GSI1PK: "string",
        GSI1SK: "string",
        GSI2PK: "string",
        GSI2SK: "string",
        GSI3PK: "string",
        GSI3SK: "string",
        GSI4PK: "string",
        GSI4SK: "string",
      },
      primaryIndex: { hashKey: "PK", rangeKey: "SK" },
      globalIndexes: {
        GSI1: { hashKey: "GSI1PK", rangeKey: "GSI1SK" },
        GSI2: { hashKey: "GSI2PK", rangeKey: "GSI2SK" },
        GSI3: { hashKey: "GSI3PK", rangeKey: "GSI3SK" },
        GSI4: { hashKey: "GSI4PK", rangeKey: "GSI4SK" },
      },
    });

    const api = new sst.aws.Function("Api", {
      handler: "bootstrap",
      bundle: "target/lambda/demo",
      architecture: "arm64",
      runtime: "provided.al2023",
      url: true,
      link: [table],
      environment: {
        PrimaryTable: table.name,
      },
    });

    return {
      api: api.url,
      table: table.name,
    };
  },
});
