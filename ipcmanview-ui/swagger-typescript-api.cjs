const { generateApi } = require("swagger-typescript-api");
const path = require("path");
const fs = require("fs");

/* NOTE: all fields are optional expect one of `output`, `url`, `spec` */
generateApi({
  name: "models.ts",
  // set to `false` to prevent the tool from writing to disk
  output: path.resolve(process.cwd(), "./src/data/station"),
  input: path.resolve(process.cwd(), "../ipcmanview-station/swagger.json"),
  generateClient: false,
  primitiveTypeConstructs: (constructs) => ({
    ...constructs,
    string: {
      "date-time": "Date | string",
    },
  }),
}).catch((e) => console.error(e));
