-- Collapse interior doubled path separators in stored project roots and
-- thread working directories. A leading UNC/network prefix (// or \\) is
-- preserved. Nested REPLACE handles runs such as //// -> /.
UPDATE project_roots
SET path = CASE
    WHEN substr(path, 1, 2) IN ('//', '\\') THEN
        substr(path, 1, 2) ||
        replace(replace(replace(replace(replace(replace(
            substr(path, 3),
            '//', '/'), '\\', '\'), '/\', '/'), '\/', '/'),
            '//', '/'), '\\', '\')
    ELSE
        replace(replace(replace(replace(replace(replace(
            path,
            '//', '/'), '\\', '\'), '/\', '/'), '\/', '/'),
            '//', '/'), '\\', '\')
END
WHERE instr(path, '//') > 0
   OR instr(path, '\\') > 0
   OR instr(path, '/\') > 0
   OR instr(path, '\/') > 0;

UPDATE threads
SET cwd = CASE
    WHEN substr(cwd, 1, 2) IN ('//', '\\') THEN
        substr(cwd, 1, 2) ||
        replace(replace(replace(replace(replace(replace(
            substr(cwd, 3),
            '//', '/'), '\\', '\'), '/\', '/'), '\/', '/'),
            '//', '/'), '\\', '\')
    ELSE
        replace(replace(replace(replace(replace(replace(
            cwd,
            '//', '/'), '\\', '\'), '/\', '/'), '\/', '/'),
            '//', '/'), '\\', '\')
END
WHERE instr(cwd, '//') > 0
   OR instr(cwd, '\\') > 0
   OR instr(cwd, '/\') > 0
   OR instr(cwd, '\/') > 0;
