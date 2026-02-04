"""Entry point for claudechic CLI."""

import argparse
import os
import sys
from importlib.metadata import version

from claudechic.app import ChatApp
from claudechic.errors import setup_logging

# Set up file logging to ~/claudechic.log
setup_logging()


def main():
    parser = argparse.ArgumentParser(description="Claude Chic")
    parser.add_argument(
        "--version",
        "-V",
        action="version",
        version=f"claudechic {version('claudechic')}",
    )
    parser.add_argument(
        "--resume", "-r", action="store_true", help="Resume the most recent session"
    )
    parser.add_argument(
        "--session", "-s", type=str, help="Resume a specific session ID"
    )
    parser.add_argument(
        "--remote-port",
        type=int,
        default=int(os.environ.get("CLAUDECHIC_REMOTE_PORT", "0")),
        help="Start HTTP server for remote control on this port",
    )
    parser.add_argument(
        "--dangerously-skip-permissions",
        "--yolo",
        action="store_true",
        help="Auto-approve all tool uses without prompting (use in sandboxed environments)",
    )
    parser.add_argument("prompt", nargs="*", help="Initial prompt to send")
    args = parser.parse_args()

    initial_prompt = " ".join(args.prompt) if args.prompt else None

    # Pass resume flag or specific session ID - actual lookup happens in app
    resume_id = (
        args.session if args.session else ("__most_recent__" if args.resume else None)
    )

    # Set terminal window title (before Textual takes over stdout)
    from pathlib import Path
    from rich.console import Console
    from rich.control import Control

    Console().control(Control.title(f"Claude Chic · {Path.cwd().name}"))

    try:
        app = ChatApp(
            resume_session_id=resume_id,
            initial_prompt=initial_prompt,
            remote_port=args.remote_port,
            skip_permissions=args.dangerously_skip_permissions,
        )
        app.run()
    except (KeyboardInterrupt, SystemExit):
        pass
    except Exception:
        import tempfile
        import traceback

        crash_log = Path(tempfile.gettempdir()) / "claudechic-crash.log"
        with open(crash_log, "w", encoding="utf-8") as f:
            traceback.print_exc(file=f)
        # Print standard traceback (not rich's fancy one) and exit
        traceback.print_exc()
        sys.exit(1)
    finally:
        # Windows: suppress stderr during interpreter shutdown to silence asyncio
        # transport __del__ exceptions (ValueError from closed pipes). See issue #31.
        # This must happen after app.run() returns but before Python garbage collects.
        if sys.platform == "win32":
            sys.stderr = open(os.devnull, "w")


if __name__ == "__main__":
    main()
