const { generateApi } = require("swagger-typescript-api");
const path = require("path");
const { execSync } = require("child_process");

const modelsDirectory = path.resolve(process.cwd(), "./src/data/station");
const modelsName = "models.ts";

/* NOTE: all fields are optional expect one of `output`, `url`, `spec` */
generateApi({
  name: modelsName,
  // set to `false` to prevent the tool from writing to disk
  output: modelsDirectory,
  input: path.resolve(process.cwd(), "../ipcmanview-station/swagger.json"),
  generateClient: false,
  primitiveTypeConstructs: (constructs) => ({
    ...constructs,
    string: {
      "date-time": "Date | string",
    },
  }),
})
  .then(() => {
    const replaceInterfaceAsTypeCommand = `awk -i inplace '/interface/ { gsub(/interface/, "type"); gsub(/{/, "= {");} 1' ${path.join(
      modelsDirectory,
      modelsName
    )}`;
    execSync(replaceInterfaceAsTypeCommand);
  })
  .catch((e) => console.error(e));
