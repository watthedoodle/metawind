pub fn patch_file(content: String, filename: String) {
    /*
        use regex to capture the data-metawind data atttribute
        and then parse as a Yaml structure.

        we can then split the content into lines and locate the
        target.

        Once the targetted line is located we can hopefully use regex replace
        to replace the css

        finally we can spit the new patched file back to the filesystem

        (?<=span data-metawind=")(.|\n)*(?="\/>)/gm
    */

}