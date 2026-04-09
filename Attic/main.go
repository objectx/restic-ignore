package main

import (
	"io/ioutil"
	"os"
	"path/filepath"

	"github.com/rs/zerolog"
	"github.com/rs/zerolog/log"
	"github.com/spf13/cobra"
	"github.com/spf13/viper"
)

func init() {
	cobra.OnInitialize(initConfig)
}

func main() {
	c := newRootCommand()
	err := c.Execute()
	if err != nil {
		log.Error().Err(err).Send()
		os.Exit(1)
	}
	os.Exit(0)
}

func newRootCommand() *cobra.Command {
	r := cobra.Command{
		Use:   "restic-ignore [flags] [<directory>]",
		Short: "Place a tag file for ignoring from restic backup",
		RunE:  doMarkIgnore,
		PersistentPreRunE: func(cmd *cobra.Command, args []string) error {
			var err error
			f := cmd.Flags()
			verbose, err := f.GetCount("verbose")
			if err != nil {
				return err
			}
			switch verbose {
			case 0:
				zerolog.SetGlobalLevel(zerolog.WarnLevel)
			case 1:
				zerolog.SetGlobalLevel(zerolog.InfoLevel)
			case 2:
				zerolog.SetGlobalLevel(zerolog.DebugLevel)
			default:
				zerolog.SetGlobalLevel(zerolog.TraceLevel)
			}
			return nil
		},
	}
	f := r.Flags()
	f.BoolP("dry-run", "n", false, "Don't modify anything")
	viper.SetDefault("dry-run", false)
	_ = viper.BindPFlag("dry-run", f.Lookup("dry-run"))
	f.CountP("verbose", "v", "Be verbose")
	viper.SetDefault("verbose", 0)
	_ = viper.BindPFlag("verbose", f.Lookup("verbose"))
	return &r
}

func doMarkIgnore(cmd *cobra.Command, args []string) error {
	var err error
	f := cmd.Flags()
	dryRun, err := f.GetBool("dry-run")
	if err != nil {
		return err
	}
	log.Debug().Strs("args", args).Send()
	for _, d := range args {
		log.Debug().Str("directory", d).Send()
		if !dryRun {
			err = os.MkdirAll(d, os.ModePerm)
			if err != nil {
				log.Error().
					Err(err).
					Str("directory", d).
					Msg("failed to create a directory")
			}
			err = createTagFile(d)
			if err != nil {
				return err
			}
		}
	}
	return nil
}

func initConfig() {
	zerolog.SetGlobalLevel(zerolog.WarnLevel)
}

func createTagFile(d string) error {
	p := filepath.Join(d, ".RESTIC-IGNORE")
	log.Info().Str("path", p).Msg("create")
	err := ioutil.WriteFile(p, []byte("restic-ignore: 58B12CA6-717F-4DA1-894A-C3126D8DFB2E"), 0644)
	if err != nil {
		log.Error().Err(err).Str("path", p).Msg("failed to create file")
		return err
	}
	return nil
}
