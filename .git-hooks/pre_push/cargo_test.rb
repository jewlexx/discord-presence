module Overcommit::Hook::PrePush
  # Runs `cargo test` before push
  #
  class CargoTest < Base
    def run
      result = execute(command)
      return :pass if result.success?

      output = result.stdout + result.stderr
      [:fail, output]
    end
  end
end
