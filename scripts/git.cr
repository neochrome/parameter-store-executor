class Git
  def initialize(@dir : String)
  end

  def uncommitted?
    s = run "diff", "--exit-code"
    s.exit_code != 0
  end

  def add(*files : String)
    s = run "add", *files
    raise "Add failed" unless s.success?
  end

  def commit(message : String)
    s = run "commit", "-m", message
    raise "Commit failed" unless s.success?
  end

  def tag(name : String)
    s = run "tag", name
    raise "Tag failed" unless s.success?
  end

  private def run(command, *args)
    Process.run "git", args: [command] + args.to_a, chdir: @dir
  end
end
