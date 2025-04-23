import os
import fnmatch
import sys
from pathlib import Path


def read_ignore_patterns(ignore_file='.cursorignore'):
    """Ignore dosyasından desenleri okur"""
    patterns = []
    if os.path.exists(ignore_file):
        with open(ignore_file, 'r', encoding='utf-8') as f:
            for line in f:
                line = line.strip()
                if line and not line.startswith('#'):
                    patterns.append(line)
    return patterns


def should_ignore(path, ignore_patterns):
    """Belirtilen yolun ignore edilip edilmeyeceğini kontrol eder"""
    path_str = str(path)
    basename = os.path.basename(path_str)
    
    # Git klasörünü kontrol et
    if basename == '.git' or '.git' in path_str or '/git' in path_str or '.venv' in path_str or '/venv' in path_str:
        return True
    
    # Log dosyalarını kontrol et
    if basename.endswith('.log') or basename == 'logs' or basename == 'log':
        return True
    
    # Specifically check for crates and assets root directories
    if basename == 'crates' or basename == 'target':
        return True
    
    for pattern in ignore_patterns:
        # Tam eşleşme
        if pattern == basename:
            return True
        
        # Joker karakter ile eşleşme
        if fnmatch.fnmatch(basename, pattern):
            return True
        
        # Tam yol için joker karakter ile eşleşme
        if fnmatch.fnmatch(path_str, pattern):
            return True
        
        # Dizin desenleri için özel kontrol
        if pattern.startswith('/') or pattern.startswith('./'):
            rel_pattern = pattern[1:] if pattern.startswith('/') else pattern[2:]
            if path_str.endswith(rel_pattern) or path_str.endswith(rel_pattern + '/'):
                return True
        
        # Klasör içeriği deseni için kontrol (örn: logs/)
        if pattern.endswith('/'):
            folder_name = pattern[:-1]
            if folder_name in path_str.split(os.sep):
                return True
    
    return False


def generate_tree(file, start_path='.', ignore_patterns=None, prefix='', is_last=True, max_depth=None, current_depth=0):
    """Dizin ağacını oluşturur ve dosyaya yazar"""
    if ignore_patterns is None:
        ignore_patterns = []
    
    if max_depth is not None and current_depth > max_depth:
        return

    # Klasör veya dosya ismini al
    path = Path(start_path)
    name = path.name if path.name else path
    
    # .git klasörünü veya log dosyalarını atla
    if should_ignore(path, ignore_patterns):
        return
    
    # Önek ve dosya/klasör ismini dosyaya yaz
    connector = '└── ' if is_last else '├── '
    file.write(f"{prefix}{connector}{name}\n")
    
    # Sonraki seviye için önek oluştur
    next_prefix = prefix + ('    ' if is_last else '│   ')
    
    if path.is_dir():
        # Dizindeki tüm öğeleri listele ve sırala
        try:
            items = []
            for item in path.iterdir():
                if not should_ignore(item, ignore_patterns):
                    items.append(item)
            
            items.sort(key=lambda x: (not x.is_dir(), x.name.lower()))
            
            # Her bir öğe için ağacı oluştur
            for i, item in enumerate(items):
                is_last_item = i == len(items) - 1
                generate_tree(file, item, ignore_patterns, next_prefix, is_last_item, max_depth, current_depth + 1)
        except PermissionError:
            file.write(f"{next_prefix}└── <erişim reddedildi>\n")


def main():
    # Komut satırı parametrelerini işle
    start_path = '.'
    max_depth = None
    output_file = "project_tree.txt"
    
    if len(sys.argv) > 1:
        start_path = sys.argv[1]
    
    if len(sys.argv) > 2:
        try:
            max_depth = int(sys.argv[2])
        except ValueError:
            print(f"Hata: Derinlik bir sayı olmalıdır, '{sys.argv[2]}' verildi")
            sys.exit(1)
    
    if len(sys.argv) > 3:
        output_file = sys.argv[3]
    
    # Ignore desenlerini oku
    ignore_patterns = read_ignore_patterns()
    
    # Ekstra ignore desenleri ekle
    extra_patterns = [
        '.git', '.git/', '/git', '*.log', 'log', 'logs', 'log/', 'logs/',
        'syserr', 'syslog', '*.log', 
        'data', 'data/', '/data', '/data/', 'docs', 'docs/', '/docs', '/docs/', 
        'examples', 'examples/', '/examples', '/examples/',
        'crates', 'crates/', '/crates', '/crates/'
    ]
    ignore_patterns.extend(extra_patterns)
    
    # Dosyayı aç ve ağacı yaz
    with open(output_file, 'w', encoding='utf-8') as f:
        # Başlık yaz
        f.write(f"Proje Ağacı: {os.path.abspath(start_path)}\n")
        f.write("=" * 40 + "\n")
        
        # Ağaç oluştur
        generate_tree(f, start_path, ignore_patterns, max_depth=max_depth)
    
    print(f"Proje ağacı '{output_file}' dosyasına yazıldı.")


if __name__ == "__main__":
    main()