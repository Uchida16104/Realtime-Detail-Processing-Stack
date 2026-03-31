import json
import sys
from collections import Counter

def main() -> None:
    raw = sys.stdin.read()
    try:
        data = json.loads(raw) if raw.strip() else {}
    except json.JSONDecodeError:
        data = {"raw": raw}

    text = json.dumps(data, ensure_ascii=False)
    chars = Counter(text)
    result = {
        "tool": "python3",
        "status": "ok",
        "input_length": len(text),
        "unique_characters": len(chars),
        "top_characters": chars.most_common(5),
        "summary": "python analysis complete"
    }
    sys.stdout.write(json.dumps(result, ensure_ascii=False))

if __name__ == "__main__":
    main()
