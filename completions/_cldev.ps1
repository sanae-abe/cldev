
using namespace System.Management.Automation
using namespace System.Management.Automation.Language

Register-ArgumentCompleter -Native -CommandName 'cldev' -ScriptBlock {
    param($wordToComplete, $commandAst, $cursorPosition)

    $commandElements = $commandAst.CommandElements
    $command = @(
        'cldev'
        for ($i = 1; $i -lt $commandElements.Count; $i++) {
            $element = $commandElements[$i]
            if ($element -isnot [StringConstantExpressionAst] -or
                $element.StringConstantType -ne [StringConstantType]::BareWord -or
                $element.Value.StartsWith('-') -or
                $element.Value -eq $wordToComplete) {
                break
        }
        $element.Value
    }) -join ';'

    $completions = @(switch ($command) {
        'cldev' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('config', 'config', [CompletionResultType]::ParameterValue, 'Configuration management commands')
            [CompletionResult]::new('dev', 'dev', [CompletionResultType]::ParameterValue, 'Development workflow commands')
            [CompletionResult]::new('git', 'git', [CompletionResultType]::ParameterValue, 'Git operation commands')
            [CompletionResult]::new('quality', 'quality', [CompletionResultType]::ParameterValue, 'Code quality commands')
            [CompletionResult]::new('tech', 'tech', [CompletionResultType]::ParameterValue, 'Tech stack specific commands')
            [CompletionResult]::new('ops', 'ops', [CompletionResultType]::ParameterValue, 'Operations commands')
            [CompletionResult]::new('analysis', 'analysis', [CompletionResultType]::ParameterValue, 'Analysis and review commands')
            [CompletionResult]::new('lr', 'lr', [CompletionResultType]::ParameterValue, 'Learning record commands')
            [CompletionResult]::new('todo', 'todo', [CompletionResultType]::ParameterValue, 'Todo management commands')
            [CompletionResult]::new('completions', 'completions', [CompletionResultType]::ParameterValue, 'Generate shell completions')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cldev;config' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Initialize cldev configuration')
            [CompletionResult]::new('check', 'check', [CompletionResultType]::ParameterValue, 'Check configuration health')
            [CompletionResult]::new('edit', 'edit', [CompletionResultType]::ParameterValue, 'Edit configuration file')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all configurations')
            [CompletionResult]::new('maintain', 'maintain', [CompletionResultType]::ParameterValue, 'Maintain configuration files')
            [CompletionResult]::new('update-docs', 'update-docs', [CompletionResultType]::ParameterValue, 'Update documentation')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cldev;config;init' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Skip interactive prompts and use defaults')
            [CompletionResult]::new('--defaults', '--defaults', [CompletionResultType]::ParameterName, 'Skip interactive prompts and use defaults')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Force initialization even if config exists')
            [CompletionResult]::new('--force', '--force', [CompletionResultType]::ParameterName, 'Force initialization even if config exists')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;config;check' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Perform detailed validation')
            [CompletionResult]::new('--detailed', '--detailed', [CompletionResultType]::ParameterName, 'Perform detailed validation')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Fix issues automatically if possible')
            [CompletionResult]::new('--fix', '--fix', [CompletionResultType]::ParameterName, 'Fix issues automatically if possible')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;config;edit' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;config;list' {
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Filter by configuration type')
            [CompletionResult]::new('--filter', '--filter', [CompletionResultType]::ParameterName, 'Filter by configuration type')
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Show detailed information')
            [CompletionResult]::new('--detailed', '--detailed', [CompletionResultType]::ParameterName, 'Show detailed information')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;config;maintain' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'Backup configurations before maintenance')
            [CompletionResult]::new('--backup', '--backup', [CompletionResultType]::ParameterName, 'Backup configurations before maintenance')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Clean up old backups')
            [CompletionResult]::new('--cleanup', '--cleanup', [CompletionResultType]::ParameterName, 'Clean up old backups')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;config;update-docs' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Validate documentation after update')
            [CompletionResult]::new('--validate', '--validate', [CompletionResultType]::ParameterName, 'Validate documentation after update')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;config;help' {
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Initialize cldev configuration')
            [CompletionResult]::new('check', 'check', [CompletionResultType]::ParameterValue, 'Check configuration health')
            [CompletionResult]::new('edit', 'edit', [CompletionResultType]::ParameterValue, 'Edit configuration file')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all configurations')
            [CompletionResult]::new('maintain', 'maintain', [CompletionResultType]::ParameterValue, 'Maintain configuration files')
            [CompletionResult]::new('update-docs', 'update-docs', [CompletionResultType]::ParameterValue, 'Update documentation')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cldev;config;help;init' {
            break
        }
        'cldev;config;help;check' {
            break
        }
        'cldev;config;help;edit' {
            break
        }
        'cldev;config;help;list' {
            break
        }
        'cldev;config;help;maintain' {
            break
        }
        'cldev;config;help;update-docs' {
            break
        }
        'cldev;config;help;help' {
            break
        }
        'cldev;dev' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('urgent', 'urgent', [CompletionResultType]::ParameterValue, 'Emergency response for production issues (5min initial response)')
            [CompletionResult]::new('fix', 'fix', [CompletionResultType]::ParameterValue, 'Fix critical bugs (same-day resolution target)')
            [CompletionResult]::new('debug', 'debug', [CompletionResultType]::ParameterValue, 'Systematic debugging workflow')
            [CompletionResult]::new('feature', 'feature', [CompletionResultType]::ParameterValue, 'Implement new feature (requirements to test)')
            [CompletionResult]::new('refactor', 'refactor', [CompletionResultType]::ParameterValue, 'Safe refactoring (incremental execution)')
            [CompletionResult]::new('optimize', 'optimize', [CompletionResultType]::ParameterValue, 'Performance optimization (measure -> analyze -> improve)')
            [CompletionResult]::new('research', 'research', [CompletionResultType]::ParameterValue, 'Technical research and learning records')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cldev;dev;urgent' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-y', '-y', [CompletionResultType]::ParameterName, 'Skip confirmation prompts')
            [CompletionResult]::new('--yes', '--yes', [CompletionResultType]::ParameterName, 'Skip confirmation prompts')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;dev;fix' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'Create fix branch automatically')
            [CompletionResult]::new('--branch', '--branch', [CompletionResultType]::ParameterName, 'Create fix branch automatically')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;dev;debug' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose debugging output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose debugging output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;dev;feature' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Skip requirements confirmation')
            [CompletionResult]::new('--skip-confirm', '--skip-confirm', [CompletionResultType]::ParameterName, 'Skip requirements confirmation')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;dev;refactor' {
            [CompletionResult]::new('-s', '-s', [CompletionResultType]::ParameterName, 'Refactoring scope')
            [CompletionResult]::new('--scope', '--scope', [CompletionResultType]::ParameterName, 'Refactoring scope')
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;dev;optimize' {
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Focus area for optimization')
            [CompletionResult]::new('--focus', '--focus', [CompletionResultType]::ParameterName, 'Focus area for optimization')
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;dev;research' {
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;dev;help' {
            [CompletionResult]::new('urgent', 'urgent', [CompletionResultType]::ParameterValue, 'Emergency response for production issues (5min initial response)')
            [CompletionResult]::new('fix', 'fix', [CompletionResultType]::ParameterValue, 'Fix critical bugs (same-day resolution target)')
            [CompletionResult]::new('debug', 'debug', [CompletionResultType]::ParameterValue, 'Systematic debugging workflow')
            [CompletionResult]::new('feature', 'feature', [CompletionResultType]::ParameterValue, 'Implement new feature (requirements to test)')
            [CompletionResult]::new('refactor', 'refactor', [CompletionResultType]::ParameterValue, 'Safe refactoring (incremental execution)')
            [CompletionResult]::new('optimize', 'optimize', [CompletionResultType]::ParameterValue, 'Performance optimization (measure -> analyze -> improve)')
            [CompletionResult]::new('research', 'research', [CompletionResultType]::ParameterValue, 'Technical research and learning records')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cldev;dev;help;urgent' {
            break
        }
        'cldev;dev;help;fix' {
            break
        }
        'cldev;dev;help;debug' {
            break
        }
        'cldev;dev;help;feature' {
            break
        }
        'cldev;dev;help;refactor' {
            break
        }
        'cldev;dev;help;optimize' {
            break
        }
        'cldev;dev;help;research' {
            break
        }
        'cldev;dev;help;help' {
            break
        }
        'cldev;git' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('commit', 'commit', [CompletionResultType]::ParameterValue, 'Create conventional commit')
            [CompletionResult]::new('branch', 'branch', [CompletionResultType]::ParameterValue, 'Create conventional branch')
            [CompletionResult]::new('merge-request', 'merge-request', [CompletionResultType]::ParameterValue, 'Create merge request (GitLab) or pull request (GitHub)')
            [CompletionResult]::new('status', 'status', [CompletionResultType]::ParameterValue, 'Show enhanced git status')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cldev;git;commit' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('--no-verify', '--no-verify', [CompletionResultType]::ParameterName, 'Skip pre-commit hooks')
            [CompletionResult]::new('--amend', '--amend', [CompletionResultType]::ParameterName, 'Amend previous commit')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;git;branch' {
            [CompletionResult]::new('-b', '-b', [CompletionResultType]::ParameterName, 'Branch type')
            [CompletionResult]::new('--branch-type', '--branch-type', [CompletionResultType]::ParameterName, 'Branch type')
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;git;merge-request' {
            [CompletionResult]::new('-t', '-t', [CompletionResultType]::ParameterName, 'Target branch')
            [CompletionResult]::new('--target', '--target', [CompletionResultType]::ParameterName, 'Target branch')
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Enable detailed mode')
            [CompletionResult]::new('--detailed', '--detailed', [CompletionResultType]::ParameterName, 'Enable detailed mode')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;git;status' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Show detailed branch information')
            [CompletionResult]::new('--detailed', '--detailed', [CompletionResultType]::ParameterName, 'Show detailed branch information')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;git;help' {
            [CompletionResult]::new('commit', 'commit', [CompletionResultType]::ParameterValue, 'Create conventional commit')
            [CompletionResult]::new('branch', 'branch', [CompletionResultType]::ParameterValue, 'Create conventional branch')
            [CompletionResult]::new('merge-request', 'merge-request', [CompletionResultType]::ParameterValue, 'Create merge request (GitLab) or pull request (GitHub)')
            [CompletionResult]::new('status', 'status', [CompletionResultType]::ParameterValue, 'Show enhanced git status')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cldev;git;help;commit' {
            break
        }
        'cldev;git;help;branch' {
            break
        }
        'cldev;git;help;merge-request' {
            break
        }
        'cldev;git;help;status' {
            break
        }
        'cldev;git;help;help' {
            break
        }
        'cldev;quality' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('lint', 'lint', [CompletionResultType]::ParameterValue, 'Run linter')
            [CompletionResult]::new('format', 'format', [CompletionResultType]::ParameterValue, 'Format code')
            [CompletionResult]::new('test', 'test', [CompletionResultType]::ParameterValue, 'Run tests')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cldev;quality;lint' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Auto-fix issues')
            [CompletionResult]::new('--fix', '--fix', [CompletionResultType]::ParameterName, 'Auto-fix issues')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;quality;format' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Check formatting without modifying files')
            [CompletionResult]::new('--check', '--check', [CompletionResultType]::ParameterName, 'Check formatting without modifying files')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;quality;test' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Generate coverage report')
            [CompletionResult]::new('--coverage', '--coverage', [CompletionResultType]::ParameterName, 'Generate coverage report')
            [CompletionResult]::new('-w', '-w', [CompletionResultType]::ParameterName, 'Watch mode')
            [CompletionResult]::new('--watch', '--watch', [CompletionResultType]::ParameterName, 'Watch mode')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;quality;help' {
            [CompletionResult]::new('lint', 'lint', [CompletionResultType]::ParameterValue, 'Run linter')
            [CompletionResult]::new('format', 'format', [CompletionResultType]::ParameterValue, 'Format code')
            [CompletionResult]::new('test', 'test', [CompletionResultType]::ParameterValue, 'Run tests')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cldev;quality;help;lint' {
            break
        }
        'cldev;quality;help;format' {
            break
        }
        'cldev;quality;help;test' {
            break
        }
        'cldev;quality;help;help' {
            break
        }
        'cldev;tech' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('start', 'start', [CompletionResultType]::ParameterValue, 'Start development environment')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cldev;tech;start' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Port number')
            [CompletionResult]::new('--port', '--port', [CompletionResultType]::ParameterName, 'Port number')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'Environment (development/production)')
            [CompletionResult]::new('--env', '--env', [CompletionResultType]::ParameterName, 'Environment (development/production)')
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;tech;help' {
            [CompletionResult]::new('start', 'start', [CompletionResultType]::ParameterValue, 'Start development environment')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cldev;tech;help;start' {
            break
        }
        'cldev;tech;help;help' {
            break
        }
        'cldev;ops' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('build', 'build', [CompletionResultType]::ParameterValue, 'Build project')
            [CompletionResult]::new('deploy', 'deploy', [CompletionResultType]::ParameterValue, 'Deploy project')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cldev;ops;build' {
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'Build environment')
            [CompletionResult]::new('--env', '--env', [CompletionResultType]::ParameterName, 'Build environment')
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-a', '-a', [CompletionResultType]::ParameterName, 'Analyze bundle after build')
            [CompletionResult]::new('--analyze', '--analyze', [CompletionResultType]::ParameterName, 'Analyze bundle after build')
            [CompletionResult]::new('-c', '-c', [CompletionResultType]::ParameterName, 'Clean before build')
            [CompletionResult]::new('--clean', '--clean', [CompletionResultType]::ParameterName, 'Clean before build')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;ops;deploy' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-y', '-y', [CompletionResultType]::ParameterName, 'Skip confirmation prompts')
            [CompletionResult]::new('--yes', '--yes', [CompletionResultType]::ParameterName, 'Skip confirmation prompts')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Dry run (show what would be deployed)')
            [CompletionResult]::new('--dry-run', '--dry-run', [CompletionResultType]::ParameterName, 'Dry run (show what would be deployed)')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;ops;help' {
            [CompletionResult]::new('build', 'build', [CompletionResultType]::ParameterValue, 'Build project')
            [CompletionResult]::new('deploy', 'deploy', [CompletionResultType]::ParameterValue, 'Deploy project')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cldev;ops;help;build' {
            break
        }
        'cldev;ops;help;deploy' {
            break
        }
        'cldev;ops;help;help' {
            break
        }
        'cldev;analysis' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('analyze', 'analyze', [CompletionResultType]::ParameterValue, 'Analyze project')
            [CompletionResult]::new('explain', 'explain', [CompletionResultType]::ParameterValue, 'Explain code or concept')
            [CompletionResult]::new('review-mr', 'review-mr', [CompletionResultType]::ParameterValue, 'Review merge request')
            [CompletionResult]::new('serena', 'serena', [CompletionResultType]::ParameterValue, 'Semantic code analysis (Serena MCP)')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cldev;analysis;analyze' {
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--format', '--format', [CompletionResultType]::ParameterName, 'Output format')
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Enable detailed analysis')
            [CompletionResult]::new('--detailed', '--detailed', [CompletionResultType]::ParameterName, 'Enable detailed analysis')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;analysis;explain' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'Show usage examples')
            [CompletionResult]::new('--examples', '--examples', [CompletionResultType]::ParameterName, 'Show usage examples')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Detailed explanation')
            [CompletionResult]::new('--detailed', '--detailed', [CompletionResultType]::ParameterName, 'Detailed explanation')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;analysis;review-mr' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Enable detailed review')
            [CompletionResult]::new('--detailed', '--detailed', [CompletionResultType]::ParameterName, 'Enable detailed review')
            [CompletionResult]::new('--security-focus', '--security-focus', [CompletionResultType]::ParameterName, 'Focus on security')
            [CompletionResult]::new('--performance-focus', '--performance-focus', [CompletionResultType]::ParameterName, 'Focus on performance')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;analysis;serena' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;analysis;help' {
            [CompletionResult]::new('analyze', 'analyze', [CompletionResultType]::ParameterValue, 'Analyze project')
            [CompletionResult]::new('explain', 'explain', [CompletionResultType]::ParameterValue, 'Explain code or concept')
            [CompletionResult]::new('review-mr', 'review-mr', [CompletionResultType]::ParameterValue, 'Review merge request')
            [CompletionResult]::new('serena', 'serena', [CompletionResultType]::ParameterValue, 'Semantic code analysis (Serena MCP)')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cldev;analysis;help;analyze' {
            break
        }
        'cldev;analysis;help;explain' {
            break
        }
        'cldev;analysis;help;review-mr' {
            break
        }
        'cldev;analysis;help;serena' {
            break
        }
        'cldev;analysis;help;help' {
            break
        }
        'cldev;lr' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('find', 'find', [CompletionResultType]::ParameterValue, 'Find learning records')
            [CompletionResult]::new('stats', 'stats', [CompletionResultType]::ParameterValue, 'Show learning statistics')
            [CompletionResult]::new('problems', 'problems', [CompletionResultType]::ParameterValue, 'List unsolved problems')
            [CompletionResult]::new('new', 'new', [CompletionResultType]::ParameterValue, 'Create new learning record')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cldev;lr;find' {
            [CompletionResult]::new('-f', '-f', [CompletionResultType]::ParameterName, 'Search in specific field')
            [CompletionResult]::new('--field', '--field', [CompletionResultType]::ParameterName, 'Search in specific field')
            [CompletionResult]::new('-l', '-l', [CompletionResultType]::ParameterName, 'Limit results')
            [CompletionResult]::new('--limit', '--limit', [CompletionResultType]::ParameterName, 'Limit results')
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;lr;stats' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Time period for statistics')
            [CompletionResult]::new('--period', '--period', [CompletionResultType]::ParameterName, 'Time period for statistics')
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-d', '-d', [CompletionResultType]::ParameterName, 'Show detailed breakdown')
            [CompletionResult]::new('--detailed', '--detailed', [CompletionResultType]::ParameterName, 'Show detailed breakdown')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;lr;problems' {
            [CompletionResult]::new('-p', '-p', [CompletionResultType]::ParameterName, 'Priority filter')
            [CompletionResult]::new('--priority', '--priority', [CompletionResultType]::ParameterName, 'Priority filter')
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-r', '-r', [CompletionResultType]::ParameterName, 'Show only recent problems')
            [CompletionResult]::new('--recent', '--recent', [CompletionResultType]::ParameterName, 'Show only recent problems')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;lr;new' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-e', '-e', [CompletionResultType]::ParameterName, 'Open editor immediately')
            [CompletionResult]::new('--edit', '--edit', [CompletionResultType]::ParameterName, 'Open editor immediately')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;lr;help' {
            [CompletionResult]::new('find', 'find', [CompletionResultType]::ParameterValue, 'Find learning records')
            [CompletionResult]::new('stats', 'stats', [CompletionResultType]::ParameterValue, 'Show learning statistics')
            [CompletionResult]::new('problems', 'problems', [CompletionResultType]::ParameterValue, 'List unsolved problems')
            [CompletionResult]::new('new', 'new', [CompletionResultType]::ParameterValue, 'Create new learning record')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cldev;lr;help;find' {
            break
        }
        'cldev;lr;help;stats' {
            break
        }
        'cldev;lr;help;problems' {
            break
        }
        'cldev;lr;help;new' {
            break
        }
        'cldev;lr;help;help' {
            break
        }
        'cldev;todo' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('manage', 'manage', [CompletionResultType]::ParameterValue, 'Manage todos')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cldev;todo;manage' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;todo;help' {
            [CompletionResult]::new('manage', 'manage', [CompletionResultType]::ParameterValue, 'Manage todos')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cldev;todo;help;manage' {
            break
        }
        'cldev;todo;help;help' {
            break
        }
        'cldev;completions' {
            [CompletionResult]::new('--lang', '--lang', [CompletionResultType]::ParameterName, 'Set language (ja/en)')
            [CompletionResult]::new('-i', '-i', [CompletionResultType]::ParameterName, 'Print installation instructions')
            [CompletionResult]::new('--install', '--install', [CompletionResultType]::ParameterName, 'Print installation instructions')
            [CompletionResult]::new('-v', '-v', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('--verbose', '--verbose', [CompletionResultType]::ParameterName, 'Enable verbose output')
            [CompletionResult]::new('-q', '-q', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--quiet', '--quiet', [CompletionResultType]::ParameterName, 'Suppress non-error output')
            [CompletionResult]::new('--no-color', '--no-color', [CompletionResultType]::ParameterName, 'Disable colored output')
            [CompletionResult]::new('-h', '-h', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('--help', '--help', [CompletionResultType]::ParameterName, 'Print help (see more with ''--help'')')
            [CompletionResult]::new('-V', '-V ', [CompletionResultType]::ParameterName, 'Print version')
            [CompletionResult]::new('--version', '--version', [CompletionResultType]::ParameterName, 'Print version')
            break
        }
        'cldev;help' {
            [CompletionResult]::new('config', 'config', [CompletionResultType]::ParameterValue, 'Configuration management commands')
            [CompletionResult]::new('dev', 'dev', [CompletionResultType]::ParameterValue, 'Development workflow commands')
            [CompletionResult]::new('git', 'git', [CompletionResultType]::ParameterValue, 'Git operation commands')
            [CompletionResult]::new('quality', 'quality', [CompletionResultType]::ParameterValue, 'Code quality commands')
            [CompletionResult]::new('tech', 'tech', [CompletionResultType]::ParameterValue, 'Tech stack specific commands')
            [CompletionResult]::new('ops', 'ops', [CompletionResultType]::ParameterValue, 'Operations commands')
            [CompletionResult]::new('analysis', 'analysis', [CompletionResultType]::ParameterValue, 'Analysis and review commands')
            [CompletionResult]::new('lr', 'lr', [CompletionResultType]::ParameterValue, 'Learning record commands')
            [CompletionResult]::new('todo', 'todo', [CompletionResultType]::ParameterValue, 'Todo management commands')
            [CompletionResult]::new('completions', 'completions', [CompletionResultType]::ParameterValue, 'Generate shell completions')
            [CompletionResult]::new('help', 'help', [CompletionResultType]::ParameterValue, 'Print this message or the help of the given subcommand(s)')
            break
        }
        'cldev;help;config' {
            [CompletionResult]::new('init', 'init', [CompletionResultType]::ParameterValue, 'Initialize cldev configuration')
            [CompletionResult]::new('check', 'check', [CompletionResultType]::ParameterValue, 'Check configuration health')
            [CompletionResult]::new('edit', 'edit', [CompletionResultType]::ParameterValue, 'Edit configuration file')
            [CompletionResult]::new('list', 'list', [CompletionResultType]::ParameterValue, 'List all configurations')
            [CompletionResult]::new('maintain', 'maintain', [CompletionResultType]::ParameterValue, 'Maintain configuration files')
            [CompletionResult]::new('update-docs', 'update-docs', [CompletionResultType]::ParameterValue, 'Update documentation')
            break
        }
        'cldev;help;config;init' {
            break
        }
        'cldev;help;config;check' {
            break
        }
        'cldev;help;config;edit' {
            break
        }
        'cldev;help;config;list' {
            break
        }
        'cldev;help;config;maintain' {
            break
        }
        'cldev;help;config;update-docs' {
            break
        }
        'cldev;help;dev' {
            [CompletionResult]::new('urgent', 'urgent', [CompletionResultType]::ParameterValue, 'Emergency response for production issues (5min initial response)')
            [CompletionResult]::new('fix', 'fix', [CompletionResultType]::ParameterValue, 'Fix critical bugs (same-day resolution target)')
            [CompletionResult]::new('debug', 'debug', [CompletionResultType]::ParameterValue, 'Systematic debugging workflow')
            [CompletionResult]::new('feature', 'feature', [CompletionResultType]::ParameterValue, 'Implement new feature (requirements to test)')
            [CompletionResult]::new('refactor', 'refactor', [CompletionResultType]::ParameterValue, 'Safe refactoring (incremental execution)')
            [CompletionResult]::new('optimize', 'optimize', [CompletionResultType]::ParameterValue, 'Performance optimization (measure -> analyze -> improve)')
            [CompletionResult]::new('research', 'research', [CompletionResultType]::ParameterValue, 'Technical research and learning records')
            break
        }
        'cldev;help;dev;urgent' {
            break
        }
        'cldev;help;dev;fix' {
            break
        }
        'cldev;help;dev;debug' {
            break
        }
        'cldev;help;dev;feature' {
            break
        }
        'cldev;help;dev;refactor' {
            break
        }
        'cldev;help;dev;optimize' {
            break
        }
        'cldev;help;dev;research' {
            break
        }
        'cldev;help;git' {
            [CompletionResult]::new('commit', 'commit', [CompletionResultType]::ParameterValue, 'Create conventional commit')
            [CompletionResult]::new('branch', 'branch', [CompletionResultType]::ParameterValue, 'Create conventional branch')
            [CompletionResult]::new('merge-request', 'merge-request', [CompletionResultType]::ParameterValue, 'Create merge request (GitLab) or pull request (GitHub)')
            [CompletionResult]::new('status', 'status', [CompletionResultType]::ParameterValue, 'Show enhanced git status')
            break
        }
        'cldev;help;git;commit' {
            break
        }
        'cldev;help;git;branch' {
            break
        }
        'cldev;help;git;merge-request' {
            break
        }
        'cldev;help;git;status' {
            break
        }
        'cldev;help;quality' {
            [CompletionResult]::new('lint', 'lint', [CompletionResultType]::ParameterValue, 'Run linter')
            [CompletionResult]::new('format', 'format', [CompletionResultType]::ParameterValue, 'Format code')
            [CompletionResult]::new('test', 'test', [CompletionResultType]::ParameterValue, 'Run tests')
            break
        }
        'cldev;help;quality;lint' {
            break
        }
        'cldev;help;quality;format' {
            break
        }
        'cldev;help;quality;test' {
            break
        }
        'cldev;help;tech' {
            [CompletionResult]::new('start', 'start', [CompletionResultType]::ParameterValue, 'Start development environment')
            break
        }
        'cldev;help;tech;start' {
            break
        }
        'cldev;help;ops' {
            [CompletionResult]::new('build', 'build', [CompletionResultType]::ParameterValue, 'Build project')
            [CompletionResult]::new('deploy', 'deploy', [CompletionResultType]::ParameterValue, 'Deploy project')
            break
        }
        'cldev;help;ops;build' {
            break
        }
        'cldev;help;ops;deploy' {
            break
        }
        'cldev;help;analysis' {
            [CompletionResult]::new('analyze', 'analyze', [CompletionResultType]::ParameterValue, 'Analyze project')
            [CompletionResult]::new('explain', 'explain', [CompletionResultType]::ParameterValue, 'Explain code or concept')
            [CompletionResult]::new('review-mr', 'review-mr', [CompletionResultType]::ParameterValue, 'Review merge request')
            [CompletionResult]::new('serena', 'serena', [CompletionResultType]::ParameterValue, 'Semantic code analysis (Serena MCP)')
            break
        }
        'cldev;help;analysis;analyze' {
            break
        }
        'cldev;help;analysis;explain' {
            break
        }
        'cldev;help;analysis;review-mr' {
            break
        }
        'cldev;help;analysis;serena' {
            break
        }
        'cldev;help;lr' {
            [CompletionResult]::new('find', 'find', [CompletionResultType]::ParameterValue, 'Find learning records')
            [CompletionResult]::new('stats', 'stats', [CompletionResultType]::ParameterValue, 'Show learning statistics')
            [CompletionResult]::new('problems', 'problems', [CompletionResultType]::ParameterValue, 'List unsolved problems')
            [CompletionResult]::new('new', 'new', [CompletionResultType]::ParameterValue, 'Create new learning record')
            break
        }
        'cldev;help;lr;find' {
            break
        }
        'cldev;help;lr;stats' {
            break
        }
        'cldev;help;lr;problems' {
            break
        }
        'cldev;help;lr;new' {
            break
        }
        'cldev;help;todo' {
            [CompletionResult]::new('manage', 'manage', [CompletionResultType]::ParameterValue, 'Manage todos')
            break
        }
        'cldev;help;todo;manage' {
            break
        }
        'cldev;help;completions' {
            break
        }
        'cldev;help;help' {
            break
        }
    })

    $completions.Where{ $_.CompletionText -like "$wordToComplete*" } |
        Sort-Object -Property ListItemText
}
