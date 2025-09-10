import json
from collections import defaultdict, Counter
import re

def analyze_submodule_report(report_path):
    with open(report_path, 'r') as f:
        report = json.load(f)

    successful_repos = len(report['repositories'])
    failed_repos = len(report['failed_repositories'])

    print(f"--- Submodule Report Analysis ---")
    print(f"Total successful repositories: {successful_repos}")
    print(f"Total failed repositories: {failed_repos}")

    # Identify duplicate repository entries (by URL)
    repo_urls = defaultdict(list)
    for url, info in report['repositories'].items():
        repo_urls[url].append(info['path'])

    duplicate_urls = {url: paths for url, paths in repo_urls.items() if len(paths) > 1}
    if duplicate_urls:
        print("\n--- Duplicate Repository URLs ---")
        for url, paths in duplicate_urls.items():
            print(f"URL: {url}")
            for path in paths:
                print(f"  - {path}")
    else:
        print("\n--- No Duplicate Repository URLs Found ---")

    # Analysis of most frequently mentioned organizations (from repository URLs)
    organizations = []
    for url in report['repositories'].keys():
        match = re.match(r'https://github.com/([^/]+)/.*', url)
        if match:
            organizations.append(match.group(1))
    
    for failed_repo in report['failed_repositories']:
        match = re.match(r'https://github.com/([^/]+)/.*', failed_repo['error'])
        if match:
            organizations.append(match.group(1))

    if organizations:
        org_counts = Counter(organizations)
        print("\n--- Most Frequently Mentioned Organizations ---")
        for org, count in org_counts.most_common(10):
            print(f"{org}: {count}")
    else:
        print("\n--- No Organizations Found ---")

    # Analysis of most frequently mentioned names or strings (submodule names, repository names)
    all_names = []
    for url, info in report['repositories'].items():
        # Add repository name from URL
        repo_name_match = re.match(r'https://github.com/[^/]+/([^/]+).*', url)
        if repo_name_match:
            all_names.append(repo_name_match.group(1).replace('.git', ''))
        
        # Add submodule names
        for submodule in info['submodules']:
            all_names.append(submodule['name'])
            if 'nested_repo' in submodule and submodule['nested_repo']:
                nested_repo_name_match = re.match(r'https://github.com/[^/]+/([^/]+).*', submodule['nested_repo']['url'])
                if nested_repo_name_match:
                    all_names.append(nested_repo_name_match.group(1).replace('.git', ''))

    if all_names:
        name_counts = Counter(all_names)
        print("\n--- Most Frequently Mentioned Repository/Submodule Names ---")
        for name, count in name_counts.most_common(10):
            print(f"{name}: {count}")
    else:
        print("\n--- No Repository/Submodule Names Found ---")

if __name__ == "__main__":
    report_file = "/data/data/com.termux.nix/files/home/pick-up-nix/source/github/meta-introspector/submodules/submodule_report_recursive_resilient.json"
    analyze_submodule_report(report_file)
