import lume from "lume/mod.ts";

const site = lume();

site.ignore(
  // TASK: Once fully migrated to Lume, merge README into top-level README and
  //       remove this.
  "README.md",
  // TASK: Move content processing to Lume.
  "content",
);

export default site;
