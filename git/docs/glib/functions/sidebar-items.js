initSidebarItems({"fn":[["access",""],["application_name",""],["assert_warning",""],["assertion_message",""],["assertion_message_cmpstr",""],["base64_decode",""],["base64_encode",""],["bit_nth_lsf",""],["bit_nth_msf",""],["bit_storage",""],["build_filenamev",""],["build_pathv",""],["canonicalize_filename",""],["chdir",""],["check_version",""],["codeset",""],["compute_checksum_for_bytes","Computes the checksum for a binary `data`. This is a convenience wrapper for [`Checksum::new()`][crate::Checksum::new()], [`Checksum::string()`][crate::Checksum::string()] and `g_checksum_free()`."],["compute_checksum_for_data","Computes the checksum for a binary `data` of `length`. This is a convenience wrapper for [`Checksum::new()`][crate::Checksum::new()], [`Checksum::string()`][crate::Checksum::string()] and `g_checksum_free()`."],["compute_checksum_for_string","Computes the checksum of a string."],["compute_hmac_for_bytes","Computes the HMAC for a binary `data`. This is a convenience wrapper for `g_hmac_new()`, `g_hmac_get_string()` and `g_hmac_unref()`."],["compute_hmac_for_data","Computes the HMAC for a binary `data` of `length`. This is a convenience wrapper for `g_hmac_new()`, `g_hmac_get_string()` and `g_hmac_unref()`."],["compute_hmac_for_string","Computes the HMAC for a string."],["console_charset",""],["dcgettext",""],["dgettext",""],["dngettext",""],["dpgettext",""],["dpgettext2",""],["environ",""],["file_open_tmp",""],["file_read_link",""],["file_set_contents",""],["file_set_contents_full",""],["file_test",""],["filename_display_basename",""],["filename_display_name",""],["format_size",""],["format_size_full",""],["host_name",""],["hostname_is_ascii_encoded",""],["hostname_is_ip_address",""],["hostname_is_non_ascii",""],["hostname_to_ascii",""],["hostname_to_unicode",""],["language_names",""],["language_names_with_category",""],["listenv",""],["locale_variants",""],["log_writer_default_set_use_stderr",""],["log_writer_default_would_drop",""],["main_current_source",""],["main_depth",""],["markup_escape_text",""],["mkdir_with_parents",""],["mkdtemp",""],["mkdtemp_full",""],["mkstemp_full",""],["monotonic_time",""],["num_processors",""],["on_error_query",""],["on_error_stack_trace",""],["os_info",""],["path_get_basename",""],["path_get_dirname",""],["path_is_absolute",""],["path_skip_root",""],["pattern_match_simple",""],["random_double",""],["random_double_range",""],["random_int",""],["random_int_range",""],["random_set_seed",""],["real_time",""],["reload_user_special_dirs_cache",""],["return_if_fail_warning",""],["rmdir",""],["set_application_name",""],["shell_parse_argv",""],["shell_quote",""],["shell_unquote",""],["spaced_primes_closest","Gets the smallest prime number from a built-in array of primes which is larger than `num`. This is used within GLib to calculate the optimum size of a `GHashTable`."],["spawn_async","Executes a child program asynchronously."],["spawn_check_exit_status","An old name for [`spawn_check_wait_status()`][crate::spawn_check_wait_status()], deprecated because its name is misleading."],["spawn_check_wait_status","Set `error` if `wait_status` indicates the child exited abnormally (e.g. with a nonzero exit code, or via a fatal signal)."],["spawn_command_line_async","A simple version of [`spawn_async()`][crate::spawn_async()] that parses a command line with [`shell_parse_argv()`][crate::shell_parse_argv()] and passes it to [`spawn_async()`][crate::spawn_async()]."],["stpcpy","Copies a nul-terminated string into the dest buffer, include the trailing nul, and return a pointer to the trailing nul byte. This is useful for concatenating multiple strings together without having to repeatedly scan for the end."],["system_config_dirs",""],["system_data_dirs",""],["unlink","A wrapper for the POSIX `unlink()` function. The `unlink()` function deletes a name from the filesystem. If this was the last link to the file and no processes have it opened, the diskspace occupied by the file is freed."],["user_cache_dir",""],["user_config_dir",""],["user_data_dir",""],["user_runtime_dir",""],["user_special_dir",""],["usleep","Pauses the current thread for the given number of microseconds."],["uuid_string_is_valid","Parses the string `str` and verify if it is a UUID."],["uuid_string_random","Generates a random UUID (RFC 4122 version 4) as a string. It has the same randomness guarantees as `GRand`, so must not be used for cryptographic purposes such as key generation, nonces, salts or one-time pads."],["warn_message","Internal function used to print messages from the public `g_warn_if_reached()` and `g_warn_if_fail()` macros."]]});