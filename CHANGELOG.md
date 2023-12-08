# net-config

# How to Work with CHANGELOG.MD
This guide explains how to work with the `CHANGELOG.md` file in our multi-repo project. We use this file for versioning and have set up a custom CI/CD pipeline to manage it effectively.

## Introduction
In our project, versioning is crucial for maintaining consistency across our repositories. We've adopted the use of the `CHANGELOG.md` file to manage versions. Our custom CI/CD pipeline ensures a smooth workflow.

## Getting Started
If you're new to changelogs, take a moment to familiarize yourself with the concept. You can read the article [Keep a Changelog](https://keepachangelog.com/en/1.1.0/) to understand the basics.

## Updating the Changelog
To update the `CHANGELOG.md` you just need to form an informative squash merge commit message. After the pull request has been merged into develop branch `CHANGELOG.md` is automatically updated by using squash merge commit message as a change log. By using this aproach developers don't need to update it manually, so keep in mind to provide a comprehensive change log while merging yoir PRs ;)