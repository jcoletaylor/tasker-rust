SET statement_timeout = 0;
SET lock_timeout = 0;
SET idle_in_transaction_session_timeout = 0;
SET client_encoding = 'UTF8';
SET standard_conforming_strings = on;
SELECT pg_catalog.set_config('search_path', '', false);
SET check_function_bodies = false;
SET xmloption = content;
SET client_min_messages = warning;
SET row_security = off;

SET default_tablespace = '';

--
-- Name: command_annotations; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.command_annotations (
    command_annotation_id bigint NOT NULL,
    command_id integer NOT NULL,
    annotation_type_id integer NOT NULL,
    annotation jsonb
);


ALTER TABLE public.command_annotations OWNER TO tasker;

--
-- Name: command_annotations_command_annotation_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.command_annotations_command_annotation_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.command_annotations_command_annotation_id_seq OWNER TO tasker;

--
-- Name: command_annotations_command_annotation_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.command_annotations_command_annotation_id_seq OWNED BY public.command_annotations.command_annotation_id;


--
-- Name: commands; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.commands (
    command_id bigint NOT NULL,
    named_command_id integer NOT NULL,
    status character varying(64) NOT NULL,
    complete boolean DEFAULT false NOT NULL,
    requested_at timestamp with time zone NOT NULL,
    initiator character varying(128),
    source_system character varying(128),
    reason character varying(128),
    bypass_steps json,
    tags jsonb,
    context jsonb,
    identity_hash character varying(128) NOT NULL
);


ALTER TABLE public.commands OWNER TO tasker;

--
-- Name: commands_command_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.commands_command_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.commands_command_id_seq OWNER TO tasker;

--
-- Name: commands_command_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.commands_command_id_seq OWNED BY public.commands.command_id;


--
-- Name: annotation_types; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.annotation_types (
    annotation_type_id integer NOT NULL,
    name character varying(64) NOT NULL,
    description character varying(255)
);


ALTER TABLE public.annotation_types OWNER TO tasker;

--
-- Name: annotation_types_annotation_type_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.annotation_types_annotation_type_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.annotation_types_annotation_type_id_seq OWNER TO tasker;

--
-- Name: annotation_types_annotation_type_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.annotation_types_annotation_type_id_seq OWNED BY public.annotation_types.annotation_type_id;


--
-- Name: dependent_system_object_maps; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dependent_system_object_maps (
    dependent_system_object_map_id bigint NOT NULL,
    dependent_system_one_id integer NOT NULL,
    dependent_system_two_id integer NOT NULL,
    remote_id_one character varying(128) NOT NULL,
    remote_id_two character varying(128) NOT NULL
);


ALTER TABLE public.dependent_system_object_maps OWNER TO tasker;

--
-- Name: dependent_system_object_maps_dependent_system_object_map_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.dependent_system_object_maps_dependent_system_object_map_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.dependent_system_object_maps_dependent_system_object_map_id_seq OWNER TO tasker;

--
-- Name: dependent_system_object_maps_dependent_system_object_map_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.dependent_system_object_maps_dependent_system_object_map_id_seq OWNED BY public.dependent_system_object_maps.dependent_system_object_map_id;


--
-- Name: dependent_systems; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.dependent_systems (
    dependent_system_id integer NOT NULL,
    name character varying(64) NOT NULL,
    description character varying(255)
);


ALTER TABLE public.dependent_systems OWNER TO tasker;

--
-- Name: dependent_systems_dependent_system_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.dependent_systems_dependent_system_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.dependent_systems_dependent_system_id_seq OWNER TO tasker;

--
-- Name: dependent_systems_dependent_system_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.dependent_systems_dependent_system_id_seq OWNED BY public.dependent_systems.dependent_system_id;

--
-- Name: named_commands; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.named_commands (
    named_command_id integer NOT NULL,
    name character varying(64) NOT NULL,
    description character varying(255)
);


ALTER TABLE public.named_commands OWNER TO tasker;

--
-- Name: named_commands_named_command_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.named_commands_named_command_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.named_commands_named_command_id_seq OWNER TO tasker;

--
-- Name: named_commands_named_command_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.named_commands_named_command_id_seq OWNED BY public.named_commands.named_command_id;


--
-- Name: named_commands_named_steps; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.named_commands_named_steps (
    id integer NOT NULL,
    named_command_id integer NOT NULL,
    named_step_id integer NOT NULL,
    skippable boolean DEFAULT false NOT NULL,
    default_retryable boolean DEFAULT true NOT NULL,
    default_retry_limit integer DEFAULT 3 NOT NULL
);


ALTER TABLE public.named_commands_named_steps OWNER TO tasker;

--
-- Name: named_commands_named_steps_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.named_commands_named_steps_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.named_commands_named_steps_id_seq OWNER TO tasker;

--
-- Name: named_commands_named_steps_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.named_commands_named_steps_id_seq OWNED BY public.named_commands_named_steps.id;


--
-- Name: named_steps; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.named_steps (
    named_step_id integer NOT NULL,
    dependent_system_id integer NOT NULL,
    name character varying(128) NOT NULL,
    description character varying(255)
);


ALTER TABLE public.named_steps OWNER TO tasker;

--
-- Name: named_steps_named_step_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.named_steps_named_step_id_seq
    AS integer
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.named_steps_named_step_id_seq OWNER TO tasker;

--
-- Name: named_steps_named_step_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.named_steps_named_step_id_seq OWNED BY public.named_steps.named_step_id;


--
-- Name: workflow_steps; Type: TABLE; Schema: public; Owner: postgres
--

CREATE TABLE public.workflow_steps (
    workflow_step_id bigint NOT NULL,
    command_id bigint NOT NULL,
    named_step_id integer NOT NULL,
    depends_on_step_id bigint,
    status character varying(64) NOT NULL,
    retryable boolean DEFAULT true NOT NULL,
    retry_limit integer DEFAULT 3,
    in_process boolean DEFAULT false NOT NULL,
    processed boolean DEFAULT false NOT NULL,
    processed_at timestamp with time zone,
    attempts integer,
    last_attempted_at timestamp with time zone,
    backoff_request_seconds integer,
    inputs jsonb,
    results jsonb
);


ALTER TABLE public.workflow_steps OWNER TO tasker;

--
-- Name: workflow_steps_workflow_step_id_seq; Type: SEQUENCE; Schema: public; Owner: postgres
--

CREATE SEQUENCE public.workflow_steps_workflow_step_id_seq
    START WITH 1
    INCREMENT BY 1
    NO MINVALUE
    NO MAXVALUE
    CACHE 1;


ALTER TABLE public.workflow_steps_workflow_step_id_seq OWNER TO tasker;

--
-- Name: workflow_steps_workflow_step_id_seq; Type: SEQUENCE OWNED BY; Schema: public; Owner: postgres
--

ALTER SEQUENCE public.workflow_steps_workflow_step_id_seq OWNED BY public.workflow_steps.workflow_step_id;


--
-- Name: command_annotations command_annotation_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.command_annotations ALTER COLUMN command_annotation_id SET DEFAULT nextval('public.command_annotations_command_annotation_id_seq'::regclass);


--
-- Name: commands command_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.commands ALTER COLUMN command_id SET DEFAULT nextval('public.commands_command_id_seq'::regclass);


--
-- Name: annotation_types annotation_type_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.annotation_types ALTER COLUMN annotation_type_id SET DEFAULT nextval('public.annotation_types_annotation_type_id_seq'::regclass);


--
-- Name: dependent_system_object_maps dependent_system_object_map_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dependent_system_object_maps ALTER COLUMN dependent_system_object_map_id SET DEFAULT nextval('public.dependent_system_object_maps_dependent_system_object_map_id_seq'::regclass);


--
-- Name: dependent_systems dependent_system_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dependent_systems ALTER COLUMN dependent_system_id SET DEFAULT nextval('public.dependent_systems_dependent_system_id_seq'::regclass);

--
-- Name: named_commands named_command_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.named_commands ALTER COLUMN named_command_id SET DEFAULT nextval('public.named_commands_named_command_id_seq'::regclass);


--
-- Name: named_commands_named_steps id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.named_commands_named_steps ALTER COLUMN id SET DEFAULT nextval('public.named_commands_named_steps_id_seq'::regclass);


--
-- Name: named_steps named_step_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.named_steps ALTER COLUMN named_step_id SET DEFAULT nextval('public.named_steps_named_step_id_seq'::regclass);


--
-- Name: workflow_steps workflow_step_id; Type: DEFAULT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.workflow_steps ALTER COLUMN workflow_step_id SET DEFAULT nextval('public.workflow_steps_workflow_step_id_seq'::regclass);


--
-- Name: command_annotations command_annotations_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.command_annotations
    ADD CONSTRAINT command_annotations_pkey PRIMARY KEY (command_annotation_id);


--
-- Name: commands commands_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.commands
    ADD CONSTRAINT commands_pkey PRIMARY KEY (command_id);


--
-- Name: annotation_types annotation_types_name_unique; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.annotation_types
    ADD CONSTRAINT annotation_types_name_unique UNIQUE (name);


--
-- Name: annotation_types annotation_types_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.annotation_types
    ADD CONSTRAINT annotation_types_pkey PRIMARY KEY (annotation_type_id);


--
-- Name: dependent_system_object_maps dependent_system_object_maps_dependent_system_one_id_dependent_; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dependent_system_object_maps
    ADD CONSTRAINT dependent_system_object_maps_dependent_system_one_id_dependent_ UNIQUE (dependent_system_one_id, dependent_system_two_id, remote_id_one, remote_id_two);


--
-- Name: dependent_system_object_maps dependent_system_object_maps_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dependent_system_object_maps
    ADD CONSTRAINT dependent_system_object_maps_pkey PRIMARY KEY (dependent_system_object_map_id);


--
-- Name: dependent_systems dependent_systems_name_unique; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dependent_systems
    ADD CONSTRAINT dependent_systems_name_unique UNIQUE (name);


--
-- Name: dependent_systems dependent_systems_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dependent_systems
    ADD CONSTRAINT dependent_systems_pkey PRIMARY KEY (dependent_system_id);

--
-- Name: named_commands named_commands_name_unique; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.named_commands
    ADD CONSTRAINT named_commands_name_unique UNIQUE (name);


--
-- Name: named_commands_named_steps named_commands_named_steps_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.named_commands_named_steps
    ADD CONSTRAINT named_commands_named_steps_pkey PRIMARY KEY (id);


--
-- Name: named_commands named_commands_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.named_commands
    ADD CONSTRAINT named_commands_pkey PRIMARY KEY (named_command_id);


--
-- Name: named_commands_named_steps named_commands_steps_ids_unique; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.named_commands_named_steps
    ADD CONSTRAINT named_commands_steps_ids_unique UNIQUE (named_command_id, named_step_id);


--
-- Name: named_steps named_step_by_system_uniq; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.named_steps
    ADD CONSTRAINT named_step_by_system_uniq UNIQUE (dependent_system_id, name);


--
-- Name: named_steps named_steps_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.named_steps
    ADD CONSTRAINT named_steps_pkey PRIMARY KEY (named_step_id);


--
-- Name: workflow_steps workflow_steps_pkey; Type: CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.workflow_steps
    ADD CONSTRAINT workflow_steps_pkey PRIMARY KEY (workflow_step_id);


--
-- Name: command_annotations_command_id_index; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX command_annotations_command_id_index ON public.command_annotations USING btree (command_id);


--
-- Name: command_annotations_annotation_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX command_annotations_annotation_idx ON public.command_annotations USING gin (annotation);


--
-- Name: command_annotations_annotation_idx1; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX command_annotations_annotation_idx1 ON public.command_annotations USING gin (annotation jsonb_path_ops);


--
-- Name: command_annotations_annotation_type_id_index; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX command_annotations_annotation_type_id_index ON public.command_annotations USING btree (annotation_type_id);


--
-- Name: commands_context_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX commands_context_idx ON public.commands USING gin (context);


--
-- Name: commands_context_idx1; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX commands_context_idx1 ON public.commands USING gin (context jsonb_path_ops);


--
-- Name: commands_identity_hash_index; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX commands_identity_hash_index ON public.commands USING btree (identity_hash);


--
-- Name: commands_named_command_id_index; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX commands_named_command_id_index ON public.commands USING btree (named_command_id);


--
-- Name: commands_requested_at_index; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX commands_requested_at_index ON public.commands USING btree (requested_at);


--
-- Name: commands_source_system_index; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX commands_source_system_index ON public.commands USING btree (source_system);


--
-- Name: commands_status_index; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX commands_status_index ON public.commands USING btree (status);


--
-- Name: commands_tags_idx; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX commands_tags_idx ON public.commands USING gin (tags);


--
-- Name: commands_tags_idx1; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX commands_tags_idx1 ON public.commands USING gin (tags jsonb_path_ops);


--
-- Name: annotation_types_name_index; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX annotation_types_name_index ON public.annotation_types USING btree (name);


--
-- Name: dependent_system_object_maps_dependent_system_one_id_index; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX dependent_system_object_maps_dependent_system_one_id_index ON public.dependent_system_object_maps USING btree (dependent_system_one_id);


--
-- Name: dependent_system_object_maps_dependent_system_two_id_index; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX dependent_system_object_maps_dependent_system_two_id_index ON public.dependent_system_object_maps USING btree (dependent_system_two_id);


--
-- Name: dependent_system_object_maps_remote_id_one_index; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX dependent_system_object_maps_remote_id_one_index ON public.dependent_system_object_maps USING btree (remote_id_one);


--
-- Name: dependent_system_object_maps_remote_id_two_index; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX dependent_system_object_maps_remote_id_two_index ON public.dependent_system_object_maps USING btree (remote_id_two);


--
-- Name: dependent_systems_name_index; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX dependent_systems_name_index ON public.dependent_systems USING btree (name);


--
-- Name: named_commands_name_index; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX named_commands_name_index ON public.named_commands USING btree (name);


--
-- Name: named_commands_named_steps_named_command_id_index; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX named_commands_named_steps_named_command_id_index ON public.named_commands_named_steps USING btree (named_command_id);


--
-- Name: named_commands_named_steps_named_step_id_index; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX named_commands_named_steps_named_step_id_index ON public.named_commands_named_steps USING btree (named_step_id);


--
-- Name: named_steps_dependent_system_id_index; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX named_steps_dependent_system_id_index ON public.named_steps USING btree (dependent_system_id);


--
-- Name: named_steps_name_index; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX named_steps_name_index ON public.named_steps USING btree (name);


--
-- Name: workflow_steps_command_id_index; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX workflow_steps_command_id_index ON public.workflow_steps USING btree (command_id);


--
-- Name: workflow_steps_depends_on_step_id_index; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX workflow_steps_depends_on_step_id_index ON public.workflow_steps USING btree (depends_on_step_id);


--
-- Name: workflow_steps_last_attempted_at_index; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX workflow_steps_last_attempted_at_index ON public.workflow_steps USING btree (last_attempted_at);


--
-- Name: workflow_steps_named_step_id_index; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX workflow_steps_named_step_id_index ON public.workflow_steps USING btree (named_step_id);


--
-- Name: workflow_steps_processed_at_index; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX workflow_steps_processed_at_index ON public.workflow_steps USING btree (processed_at);


--
-- Name: workflow_steps_status_index; Type: INDEX; Schema: public; Owner: postgres
--

CREATE INDEX workflow_steps_status_index ON public.workflow_steps USING btree (status);


--
-- Name: command_annotations command_annotations_command_id_foreign; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.command_annotations
    ADD CONSTRAINT command_annotations_command_id_foreign FOREIGN KEY (command_id) REFERENCES public.commands(command_id);


--
-- Name: command_annotations command_annotations_annotation_type_id_foreign; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.command_annotations
    ADD CONSTRAINT command_annotations_annotation_type_id_foreign FOREIGN KEY (annotation_type_id) REFERENCES public.annotation_types(annotation_type_id);


--
-- Name: commands commands_named_command_id_foreign; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.commands
    ADD CONSTRAINT commands_named_command_id_foreign FOREIGN KEY (named_command_id) REFERENCES public.named_commands(named_command_id);


--
-- Name: dependent_system_object_maps dependent_system_object_maps_dependent_system_one_id_foreign; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dependent_system_object_maps
    ADD CONSTRAINT dependent_system_object_maps_dependent_system_one_id_foreign FOREIGN KEY (dependent_system_one_id) REFERENCES public.dependent_systems(dependent_system_id);


--
-- Name: dependent_system_object_maps dependent_system_object_maps_dependent_system_two_id_foreign; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.dependent_system_object_maps
    ADD CONSTRAINT dependent_system_object_maps_dependent_system_two_id_foreign FOREIGN KEY (dependent_system_two_id) REFERENCES public.dependent_systems(dependent_system_id);


--
-- Name: named_commands_named_steps named_commands_named_steps_named_command_id_foreign; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.named_commands_named_steps
    ADD CONSTRAINT named_commands_named_steps_named_command_id_foreign FOREIGN KEY (named_command_id) REFERENCES public.named_commands(named_command_id);


--
-- Name: named_commands_named_steps named_commands_named_steps_named_step_id_foreign; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.named_commands_named_steps
    ADD CONSTRAINT named_commands_named_steps_named_step_id_foreign FOREIGN KEY (named_step_id) REFERENCES public.named_steps(named_step_id);


--
-- Name: named_steps named_steps_dependent_system_id_foreign; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.named_steps
    ADD CONSTRAINT named_steps_dependent_system_id_foreign FOREIGN KEY (dependent_system_id) REFERENCES public.dependent_systems(dependent_system_id);


--
-- Name: workflow_steps workflow_steps_command_id_foreign; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.workflow_steps
    ADD CONSTRAINT workflow_steps_command_id_foreign FOREIGN KEY (command_id) REFERENCES public.commands(command_id);


--
-- Name: workflow_steps workflow_steps_depends_on_step_id_foreign; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.workflow_steps
    ADD CONSTRAINT workflow_steps_depends_on_step_id_foreign FOREIGN KEY (depends_on_step_id) REFERENCES public.workflow_steps(workflow_step_id);


--
-- Name: workflow_steps workflow_steps_named_step_id_foreign; Type: FK CONSTRAINT; Schema: public; Owner: postgres
--

ALTER TABLE ONLY public.workflow_steps
    ADD CONSTRAINT workflow_steps_named_step_id_foreign FOREIGN KEY (named_step_id) REFERENCES public.named_steps(named_step_id);


GRANT ALL ON ALL TABLES IN SCHEMA "public" TO tasker;