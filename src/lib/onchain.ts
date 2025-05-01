// `dot` is the alias assigned when running `npx papi add`
import { dot } from "@polkadot-api/descriptors";
import { createClient } from "polkadot-api";
import { getSmProvider } from "polkadot-api/sm-provider";
import { chainSpec } from "polkadot-api/chains/polkadot";
import { start } from "polkadot-api/smoldot";

// Initialize Smoldot client
const smoldot = start();
const chain = await smoldot.addChain({ chainSpec });

// Set up a client to connect to the Polkadot relay chain
const client = createClient(getSmProvider(chain));

// Access the `TypedApi` to interact with all available chain calls and types
const typedApi = client.getTypedApi(dot);

const version = await typedApi.constants.System.Version();

const metadata = await typedApi.apis.Metadata.metadata();
