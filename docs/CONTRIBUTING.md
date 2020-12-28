# Contributing to Propagation

Thank you for your interest in contributing to Propagation!

If you are unsure about how to help with this project, we can point you in the
right direction! Below is a list of the many ways to help with Propagation's
development. Any help, big or small, is appreciated!

1. [I want to submit issues or request features.](#submitting-issues)
2. [I want to contribute code.](#pull-requests)
3. [I want to write documentation.](#writing-documentation)
4. [Are there any useful resources I can read?](#useful-resources)

**Attention:** Any interaction with the Dezzyne team, its projects, and its
contributers, is subject to our [Code of Conduct](co).
By contributing to this project you agree to abide to its terms and conditions.

[co]: https://github.com/dezzyne/propagation/blob/master/CODE_OF_CONDUCT.md

If there are any questions that you may have not been addressed by the
documentation, do not hesitate to leave open a issue or come and have your
question asked in the [Dezzyne Discord Server](di).

[di]: https://www.discordapp.com

## Submitting Issues

One of the many ways to help us with the development of Propagation is to let
us know about persistant or rare unintended behavior that may occur when using
it. Anyone can report these bugs or even go as far to request features on the
project's issue tracker. If a bug gets through our tests we won't know
about it until it occurs for a user. So let us know as soon as possible!

* [Propagation Tracker][pt]: Issue tracker for the Propagation project

[pt]: https://github.com/dezzyne/propagation/issues

Before you post your issue, please take a look through the tracker for other
issues that are posted. Your issue may have been reported by another user. In
that case, you should direction your information towards that particular post.
Also don't worry about accidently creating an duplicate issue. We can always
mark a issue as a duplicate of another one.

Propagation is built with the stable branch of Rust in mind, but is developed
using the beta and nightly builds. If any issues occur during the development
of Propagation involving Rust we will try to notify the Rust team as soon as
possible so that the problem does not get to the stable branch.

Otherwise, thank you for posting your issue! Both the Dezzyne team and the
community at large will try to fix the problem as soon as possible!

## Pull Requests

### Submission Checklist

## Writing Documentation

## Profiling the application

## Useful Resourcess

## Miscellaneous

### Glossary

### Philosophy

### Architecture

### Logging

### Linting

### Formatting

### Testing

#### Unit Testing

#### Integration Testing

#### Property Testing

### Git/GitHub workflow

We recommend following a Gitflow approach when contributing to this repository.

Otherwise, below are step by step directions for opening a PR on GitHub:

1. Fork this repository
2. Initialize a local git repository in any directory for your fork: `git init`
3. Create a remote reference to your fork: `git remote add origin my-fork-repo-url`
4. Create a branch off of `develop` for your work: `git checkout -b feature/my-feature-branch-name`
5. Make some changes, committing them along the way
6. When your changes are ready for review, push your branch: `git push origin my-feature-branch`
7. Create a pull request from your branch to `propagation/develop`
8. No need to assign the pull request to anyone, we'll review it when we can
9. When the changes have been reviewed and approved, someone will squash and merge for you
