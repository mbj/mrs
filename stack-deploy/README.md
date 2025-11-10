# stack-deploy

A CLI tool for managing AWS CloudFormation stacks with enhanced workflow support.

## Features

- **Stack Management**: Create, update, sync, and delete CloudFormation stacks
- **Change Sets**: Interactive change set review before applying updates
- **Event Watching**: Real-time monitoring of stack events during deployments
- **Template Upload**: Automatic S3 upload for large templates
- **Lambda Deployment**: Built-in Lambda function deployment utilities
- **Parameter Management**: Type-safe parameter handling
- **Secrets Integration**: AWS Secrets Manager integration

## Commands

### Instance Management

```bash
# List all registered stack instances
stack-deploy instance list

# Sync stack (create if absent, update if exists)
stack-deploy instance sync --stack-name <name> --parameter Key=Value

# Update existing stack
stack-deploy instance update --stack-name <name> --parameter Key=Value

# Delete stack
stack-deploy instance delete --stack-name <name>

# Watch stack events in real-time
stack-deploy instance watch --stack-name <name>
```

### Change Set Operations

```bash
# Create a change set
stack-deploy instance change-set --stack-name <name> create --change-set-name <cs-name> --parameter Key=Value

# List change sets
stack-deploy instance change-set --stack-name <name> list

# Describe a change set
stack-deploy instance change-set --stack-name <name> describe --change-set-name <cs-name>

# Delete a change set
stack-deploy instance change-set --stack-name <name> delete --change-set-name <cs-name>
```

## Review Modes

When updating or syncing stacks, you can control the change set review behavior:

- `--review-change-set interactive` (default): Review changes before applying
- `--review-change-set no-review`: Apply changes without review

## Integration

Works seamlessly with the [stratosphere](https://github.com/mbj/mrs/tree/main/stratosphere) library for type-safe CloudFormation template generation.

## License

Part of the [mrs](https://github.com/mbj/mrs) collection by [@mbj](https://github.com/mbj)
