You have pending loop items. The turn is not closed until each is done or explicitly cancelled:

{{#each todos}}
- [{{this.status}}] {{this.content}}
{{/each}}

Complete or cancel every pending item before finishing. Do not claim done while any remain in progress.
