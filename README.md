# 🪽 HawkOps

An unofficial CLI companion to StackHawk

## Purpose

HawkOps is a CLI tool that provides a set of commands to interact with the StackHawk API. This tool is intended to be used by developers and DevOps engineers to automate tasks that are not available in the official StackHawk UI or CLI.

## Initial Goals

Here are a few itches I'd like to scratch with this tool.

1. Setup initial authentication with StackHawk API
2. Maintain authorization - check and if necessary refresh token on each command
3. List all applications
4. List latest scans for an application
5. List users and their teams
6. Query apps for scan configs with particular parameters
7. Create other reports as needed
8. Model API queries to support StackHawk prospects and customers
9. Create a query language to support more complex queries

## Usage

`hawkops [noun-command] [verb-command] [options]`
`hawkops --help`
`hawkops [noun-command] --help`
`hawkops [noun-command] [verb-command] --help`

## Build

## Development Setup
