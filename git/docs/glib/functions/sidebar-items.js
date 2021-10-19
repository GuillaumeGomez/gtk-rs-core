initSidebarItems({"fn":[["access",""],["application_name","Gets a human-readable name for the application, as set by [`set_application_name()`][crate::set_application_name()]. This name should be localized if possible, and is intended for display to the user. Contrast with `g_get_prgname()`, which gets a non-localized name. If [`set_application_name()`][crate::set_application_name()] has not been called, returns the result of `g_get_prgname()` (which may be [`None`] if `g_set_prgname()` has also not been called)."],["assert_warning",""],["assertion_message",""],["assertion_message_cmpstr",""],["base64_decode",""],["base64_encode",""],["bit_nth_lsf",""],["bit_nth_msf",""],["bit_storage",""],["build_filenamev",""],["build_pathv",""],["canonicalize_filename",""],["chdir",""],["check_version",""],["clear_error",""],["codeset","Gets the character set for the current locale."],["compute_checksum_for_bytes",""],["compute_checksum_for_data",""],["compute_checksum_for_string",""],["compute_hmac_for_bytes",""],["compute_hmac_for_data",""],["compute_hmac_for_string",""],["console_charset","Obtains the character set used by the console attached to the process, which is suitable for printing output to the terminal."],["dcgettext",""],["dgettext",""],["dngettext",""],["dpgettext",""],["dpgettext2",""],["environ","Gets the list of environment variables for the current process."],["file_get_contents",""],["file_open_tmp",""],["file_read_link","Reads the contents of the symbolic link `filename` like the POSIX `readlink()` function. The returned string is in the encoding used for filenames. Use `g_filename_to_utf8()` to convert it to UTF-8."],["file_set_contents","Writes all of `contents` to a file named `filename`. This is a convenience wrapper around calling [`file_set_contents_full()`][crate::file_set_contents_full()] with `flags` set to `G_FILE_SET_CONTENTS_CONSISTENT | G_FILE_SET_CONTENTS_ONLY_EXISTING` and `mode` set to `0666`."],["file_set_contents_full","Writes all of `contents` to a file named `filename`, with good error checking. If a file called `filename` already exists it will be overwritten."],["file_test","Returns [`true`] if any of the tests in the bitfield `test` are [`true`]. For example, `(G_FILE_TEST_EXISTS | G_FILE_TEST_IS_DIR)` will return [`true`] if the file exists; the check whether it’s a directory doesn’t matter since the existence test is [`true`]. With the current set of available tests, there’s no point passing in more than one test at a time."],["filename_display_basename","Returns the display basename for the particular filename, guaranteed to be valid UTF-8. The display name might not be identical to the filename, for instance there might be problems converting it to UTF-8, and some files can be translated in the display."],["filename_display_name","Converts a filename into a valid UTF-8 string. The conversion is not necessarily reversible, so you should keep the original around and use the return value of this function only for display purposes. Unlike `g_filename_to_utf8()`, the result is guaranteed to be non-[`None`] even if the filename actually isn’t in the GLib file name encoding."],["format_size","Formats a size (for example the size of a file) into a human readable string. Sizes are rounded to the nearest size prefix (kB, MB, GB) and are displayed rounded to the nearest tenth. E.g. the file size 3292528 bytes will be converted into the string “3.2 MB”. The returned string is UTF-8, and may use a non-breaking space to separate the number and units, to ensure they aren’t separated when line wrapped."],["format_size_full","Formats a size."],["host_name","Return a name for the machine."],["hostname_is_ascii_encoded",""],["hostname_is_ip_address",""],["hostname_is_non_ascii",""],["hostname_to_ascii",""],["hostname_to_unicode",""],["language_names","Computes a list of applicable locale names, which can be used to e.g. construct locale-dependent filenames or search paths. The returned list is sorted from most desirable to least desirable and always contains the default locale “C”."],["language_names_with_category","Computes a list of applicable locale names with a locale category name, which can be used to construct the fallback locale-dependent filenames or search paths. The returned list is sorted from most desirable to least desirable and always contains the default locale “C”."],["listenv",""],["locale_variants","Returns a list of derived variants of `locale`, which can be used to e.g. construct locale-dependent filenames or search paths. The returned list is sorted from most desirable to least desirable. This function handles territory, charset and extra locale modifiers. See `setlocale(3)` for information about locales and their format."],["log_writer_default_set_use_stderr",""],["log_writer_default_would_drop",""],["main_current_source",""],["main_depth",""],["markup_escape_text",""],["mkdir_with_parents",""],["mkdtemp",""],["mkdtemp_full",""],["mkstemp_full",""],["monotonic_time","Queries the system monotonic time."],["num_processors","Determine the approximate number of threads that the system will schedule simultaneously for this process. This is intended to be used as a parameter to `g_thread_pool_new()` for CPU bound tasks and similar cases."],["on_error_query",""],["on_error_stack_trace",""],["os_info","Get information about the operating system."],["path_get_basename",""],["path_get_dirname",""],["path_is_absolute",""],["path_skip_root",""],["pattern_match_simple",""],["random_double",""],["random_double_range",""],["random_int",""],["random_int_range",""],["random_set_seed",""],["real_time","Queries the system wall-clock time."],["reload_user_special_dirs_cache",""],["return_if_fail_warning",""],["rmdir",""],["set_application_name",""],["shell_parse_argv",""],["shell_quote",""],["shell_unquote",""],["spaced_primes_closest",""],["spawn_async",""],["spawn_check_exit_status",""],["spawn_check_wait_status",""],["spawn_command_line_async",""],["stpcpy",""],["system_config_dirs","Returns an ordered list of base directories in which to access system-wide configuration information."],["system_data_dirs","Returns an ordered list of base directories in which to access system-wide application data."],["unlink","A wrapper for the POSIX `unlink()` function. The `unlink()` function deletes a name from the filesystem. If this was the last link to the file and no processes have it opened, the diskspace occupied by the file is freed."],["user_cache_dir","Returns a base directory in which to store non-essential, cached data specific to particular user."],["user_config_dir",""],["user_data_dir",""],["user_runtime_dir",""],["user_special_dir",""],["usleep","Pauses the current thread for the given number of microseconds."],["uuid_string_is_valid","Parses the string `str` and verify if it is a UUID."],["uuid_string_random","Generates a random UUID (RFC 4122 version 4) as a string. It has the same randomness guarantees as `GRand`, so must not be used for cryptographic purposes such as key generation, nonces, salts or one-time pads."],["warn_message","Internal function used to print messages from the public `g_warn_if_reached()` and `g_warn_if_fail()` macros."]]});